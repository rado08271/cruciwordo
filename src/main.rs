mod board;
mod dictionary;
mod entities;

use std::any::Any;
use std::process::Termination;
use axum::{Json, Router};
use axum::http::{StatusCode};
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;
use chrono::prelude::*;
use dotenv::dotenv;
use serde_json::json;
use skytable::{ClientResult, Config};
use crate::entities::create_puzzle::generate_new_board;
use crate::entities::get_puzzle::get_board_by_id;

async fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "get_board": "/api/{board_id}",
            "create_board": "/api/g"
        }))
    )
}


#[tokio::main]
async fn main() {
    // CREATE SPACE IF NOT EXISTS cruciwordo
    // CREATE MODEL IF NOT EXISTS cruciwordo.puzzle (primary id: string, created_date: string, rows: uint64, cols: uint64, original_message: string, solution: string, grid: string, history: list {type: string})
    dotenv().ok();

    let app = Router::new()
        .route("/api/{board_id}", get(get_board_by_id))
        .route("/api/g", post(generate_new_board))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
    ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
