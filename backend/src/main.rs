use actix_web::{web, App, HttpServer, middleware::Logger};
use std::io;
use tokio::signal;
use dotenvy::dotenv;

mod api_error;
mod config;
mod db;
mod http;
mod models;
mod service;

use crate::config::Config;
use crate::db::create_pool;
use crate::http::health::health_check;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("arenax_backend=info")
        .init();

    // Create database pool
    let db_pool = create_pool(&config)
        .await
        .expect("Failed to create database pool");

    tracing::info!(
        "Starting ArenaX backend server on {}:{}",
        config.server.host,
        config.server.port
    );

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(health_check))
                    // TODO: Add more routes here as implemented
            )
    })
    .bind((config.server.host.clone(), config.server.port))?
    .run();

    // Graceful shutdown
    let server_handle = server.handle();
    tokio::spawn(async move {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");
        tracing::info!("Shutdown signal received, stopping server...");
        server_handle.stop(true).await;
    });

    server.await
}