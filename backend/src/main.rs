use std::io;

mod api_error;
mod config;
mod db;
mod http;
mod models;
mod service;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("ArenaX Backend starting...");

    // TODO: Initialize server with proper configuration
    // For now, just exit successfully to test compilation

    Ok(())
}