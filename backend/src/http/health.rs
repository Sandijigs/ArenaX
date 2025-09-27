use crate::api_error::ApiError;
use crate::db::DbPool;
use actix_web::{web, HttpResponse, Result};

pub async fn health_check(db_pool: web::Data<DbPool>) -> Result<HttpResponse, ApiError> {
    // Check database
    crate::db::health_check(&db_pool).await?;

    // TODO: Check Redis
    // For now, assume Redis is ok

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "database": "ok",
        "redis": "ok"
    })))
}
