use axum::{
    routing::{get, post},
    Router,
    Json,
    response::Json as ResponseJson,
    extract::{Query, Extension},
};
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};
use tracing_subscriber;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;


mod routes;
mod models;

use routes::health;
use models::ApiResponse;

#[derive(Debug, Deserialize)]
struct HelloQuery {
    name: Option<String>,
}

#[derive(Debug, Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello_handler(Query(params): Query<HelloQuery>) -> ResponseJson<HelloResponse> {
    let name = params.name.unwrap_or_else(|| "World".to_string());
    ResponseJson(HelloResponse {
        message: format!("Hello, {}!", name),
    })
}

async fn echo_handler(Json(payload): Json<serde_json::Value>) -> ResponseJson<ApiResponse<serde_json::Value>> {
    ResponseJson(ApiResponse {
        success: true,
        data: Some(payload),
        message: Some("Echo successful".to_string()),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    tracing_subscriber::fmt::init();

    // Database connection
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(|| async { "Bloop API" }))
        .route("/health", get(health::health_check))
        .route("/hello", get(hello_handler))
        .route("/echo", post(echo_handler))
        .layer(Extension(pool))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    
    tracing::info!("Server running on http://0.0.0.0:3001");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
