use crate::api_error::ApiError;
use crate::db::DbPool;
use crate::models::user::{User, CreateUserRequest, LoginRequest, AuthResponse, UserProfile};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthService {
    pool: Arc<DbPool>,
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // user id
    exp: usize,   // expiration time
    iat: usize,   // issued at
}

impl AuthService {
    pub fn new(pool: DbPool, jwt_secret: String) -> Self {
        Self {
            pool: Arc::new(pool),
            jwt_secret,
        }
    }

    pub async fn register(&self, request: CreateUserRequest) -> Result<User, ApiError> {
        // Check if user already exists
        let existing_user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 OR username = $2"
        )
        .bind(&request.email)
        .bind(&request.username)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|_| ApiError::internal_error("Database error"))?;

        if existing_user.is_some() {
            return Err(ApiError::bad_request("User already exists"));
        }

        // Hash password
        let password_hash = hash(request.password, DEFAULT_COST)
            .map_err(|_| ApiError::internal_error("Password hashing failed"))?;

        // Create user
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&request.username)
        .bind(&request.email)
        .bind(&password_hash)
        .fetch_one(&*self.pool)
        .await
        .map_err(|_| ApiError::internal_error("Failed to create user"))?;

        Ok(user)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, ApiError> {
        // Find user
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|_| ApiError::internal_error("Database error"))?
        .ok_or_else(|| ApiError::unauthorized("Invalid credentials"))?;

        // Verify password
        let is_valid = verify(&request.password, &user.password_hash)
            .map_err(|_| ApiError::internal_error("Password verification failed"))?;

        if !is_valid {
            return Err(ApiError::unauthorized("Invalid credentials"));
        }

        // Generate tokens
        let token = self.generate_token(&user.id)?;
        let refresh_token = self.generate_refresh_token(&user.id)?;

        let profile = UserProfile {
            id: user.id,
            username: user.username,
            email: user.email,
            is_verified: user.is_verified,
            created_at: user.created_at,
        };

        Ok(AuthResponse {
            token,
            refresh_token,
            user: profile,
        })
    }

    fn generate_token(&self, user_id: &Uuid) -> Result<String, ApiError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: Utc::now().timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| ApiError::internal_error("Token generation failed"))
    }

    fn generate_refresh_token(&self, user_id: &Uuid) -> Result<String, ApiError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(30))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: Utc::now().timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| ApiError::internal_error("Refresh token generation failed"))
    }

    pub fn verify_token(&self, token: &str) -> Result<Uuid, ApiError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::unauthorized("Invalid token"))?;

        Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| ApiError::unauthorized("Invalid token"))
    }
}