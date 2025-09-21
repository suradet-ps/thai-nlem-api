// src/main.rs

mod db;
mod handlers;
mod models;
mod routes;
mod seeder;

use axum::http::{header, Method};
use std::env;
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    dotenvy::dotenv().ok();

    let pool = db::create_pool().await?;

    let args: Vec<String> = env::args().collect();
    if args.get(1) == Some(&"seed".to_string()) {
        seeder::seed_data(&pool).await?;
        return Ok(());
    }

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET])
        .allow_headers([header::CONTENT_TYPE]);

    let app = routes::create_router(pool).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("🚀 Server listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}