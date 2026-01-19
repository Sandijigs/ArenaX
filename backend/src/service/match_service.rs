use crate::api_error::ApiError;
use crate::db::DbPool;
use crate::models::match_model::{Match, MatchResult};
use sqlx::types::Uuid;

#[derive(Clone)]
pub struct MatchService {
    pool: DbPool,
}

impl MatchService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_match(&self, match_id: Uuid, user_id: Option<Uuid>) -> Result<Match, ApiError> {
        // TODO: Implement match retrieval
        Err(ApiError::not_found("Match not found"))
    }

    pub async fn report_score(
        &self,
        match_id: Uuid,
        user_id: Uuid,
        result: MatchResult,
    ) -> Result<MatchResult, ApiError> {
        // TODO: Implement score reporting
        Err(ApiError::internal_error("Match service not yet implemented"))
    }

    pub async fn get_match_history(&self, user_id: Uuid) -> Result<Vec<Match>, ApiError> {
        // TODO: Implement match history
        Ok(vec![])
    }

    pub async fn get_leaderboard(&self, game_type: String) -> Result<Vec<(Uuid, i32)>, ApiError> {
        // TODO: Implement leaderboard
        Ok(vec![])
    }
}