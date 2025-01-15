mod board;
mod dictionary;

use std::cmp::max;
use axum::{extract, Json, Router};
use axum::http::{StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde_json::{json, Value};
use serde::Deserialize;
use serde::Serialize;
use crate::board::{Board, Placement};
use tower_http::trace::TraceLayer;
use crate::dictionary::Dictionary;
use nanoid::nanoid;
use chrono::prelude::*;

#[derive(Serialize)]
struct CruciWordPuzzle {
    id: String,
    created_date: String,
    rows: usize,
    cols: usize,
    original_message: String,
    solution: String,
    grid: Vec<Vec<char>>,
    history: Vec<Placement>
}

#[derive(Deserialize,)]
struct Params {
    rows: usize,
    cols: usize,
    message: String
}

async fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "get_board": "/api/{board_id}",
            "create_board": "/api/g"
        }))
    )
}

async fn get_board_by_id (
    extract::Path(board_id): extract::Path<String>
) -> (StatusCode, Json<Value>) {

    (
        StatusCode::OK,
        Json((json!({"Status": "Not yet implemented"})))
    )
}

async fn generate_new_board (
    extract::Json(
        Params {rows, cols, message}
    ): extract::Json<Params>
) -> (StatusCode, Json<CruciWordPuzzle>) {

    let mut board: Board = Board::new(rows, cols, message.clone());
    let mut dictionary: Dictionary = Dictionary::from_file("./res/en.dr".to_string(), max(rows, cols));
    let mut placements: Vec<Placement> = Vec::new();

    while !board.is_filled() {
        let random_word = dictionary.get_random_word();
        let valid_placement = board.place_word_on_board(random_word);

        if valid_placement.is_some() {
            placements.push(valid_placement.unwrap());
        }
    }

    let result_board: CruciWordPuzzle = CruciWordPuzzle {
        id: nanoid!(10, &nanoid::alphabet::SAFE),
        created_date: Utc::now().to_rfc3339(),
        history: placements,
        original_message: message.clone(),
        grid: board.grid,
        solution: board.solution,
        rows, cols
    };

    (
        StatusCode::OK,
        Json((result_board))
    )
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/api/{board_id}", get(get_board_by_id))
        .route("/api/g", post(generate_new_board))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
    ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
