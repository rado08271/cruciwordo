mod board;
mod dictionary;
mod entities;

use std::error::Error;
use std::fmt::Debug;
use std::process::Termination;
use axum::{Json, Router};
use axum::http::{Method, StatusCode};
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use chrono::prelude::*;
use dotenv::dotenv;
use serde_json::json;
use skytable::{ClientResult, Config, Pipeline, query};
use crate::entities::create_puzzle::generate_new_board;
use crate::entities::get_database;
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

fn initialize_db() {

    let create_space = query!(
        "CREATE SPACE IF NOT EXISTS cruciwordo"
    );

    let create_model = query!(
        "CREATE MODEL IF NOT EXISTS cruciwordo.puzzle (primary id: string, created_date: string, rows: uint64, cols: uint64, original_message: string, solution: string, grid: string, history: list {type: string})"
    );

    let init_database = Pipeline::new().add(&create_space).add(&create_model);

    let db_conn = get_database();

    let pipe_result = db_conn.unwrap().execute_pipeline(&init_database);

    if (pipe_result.is_err()) {
        let pipe_result_err = pipe_result.err();
        if (pipe_result_err.is_some()) {
            let error = pipe_result_err.unwrap();
            println!("Error is string {}", error.to_string());
        }
    } else {
        println!("is OK")
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let init_database = std::env::var("SKYTABLE_INIT_DATABASE");
    if init_database.is_ok() && init_database.unwrap().parse::<bool>() == Ok(true) {
        initialize_db();
    }

    let cors_layer_restrictions = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(AllowOrigin::mirror_request())
        .allow_private_network(true);

    let app = Router::new()
        .route("/api/{board_id}", get(get_board_by_id))
        .route("/api/g", post(generate_new_board))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer_restrictions)
    ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:10000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
