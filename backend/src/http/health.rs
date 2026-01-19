use actix_web::{HttpResponse, Result};
use serde_json::json;
use crate::db::DbPool;

pub async fn health_check(pool: actix_web::web::Data<DbPool>) -> Result<HttpResponse> {
    // Check database connection
    let db_status = match crate::db::health_check(&pool).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let response = json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "database": db_status,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    Ok(HttpResponse::Ok().json(response))
}