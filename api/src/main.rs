mod puzzle_dto;

use axum::http::{Method, StatusCode};
use axum::{extract, Json, Router};
use axum::routing::{get, post};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use db::{initialize_db, insert_puzzle, get_puzzle_by_id, PuzzleDAO, PuzzleDTO};
use generator::generate_board;

async fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "get_board": "/api/{board_id}",
            "create_board": "/api/g"
        }))
    )
}

#[derive(Deserialize)]
pub struct CreateBoardParams {
    rows: usize,
    cols: usize,
    message: String
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
    status: u16
}

#[derive(Serialize)]
pub struct CreateResponse {
    id: String
}

pub fn create_board(
    extract::Json(
        CreateBoardParams {rows, cols, message}
    ): extract::Json<CreateBoardParams>
) -> Result<Json<CreateResponse>, (StatusCode, Json<ErrorResponse>)>  {
    let board = generate_board(rows, cols, message.clone());

    let board_id = insert_puzzle(board);

    if (board_id.is_ok()) {
        return Ok(
            Json(CreateResponse {
                id: board_id.unwrap()
            })
        );
    } else {
        return axum::response::Result::Err ((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                status: 400,
                message: "There was error processing request".to_string()
            })
        ))
    }
}

pub async fn get_board_by_id (
    extract::Path(board_id): extract::Path<String>
) -> Result<Json<PuzzleDTO>, (StatusCode, Json<ErrorResponse>)> {
    let mut puzzle_dto_wrapped = get_puzzle_by_id(board_id);


    if puzzle_dto_wrapped.is_err() {
        let error = puzzle_dto_wrapped.err();

        if error.is_some() {
            return axum::response::Result::Err ((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    status: 400,
                    message: error.unwrap().to_string()
                })
            ))
        }

        return axum::response::Result::Err ((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                status: 400,
                message: "There was error processing request".to_string()
            })
        ));
    }

    return Ok(
        Json(puzzle_dto_wrapped.unwrap())
    );
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
        .allow_origin([
            "http://localhost:10000".parse().unwrap(),
            "http://0.0.0.0:10000".parse().unwrap(),
            "https://cruciwordo.onrender.com".parse().unwrap(),
        ])
        .allow_private_network(true);

    let app = Router::new()
        .route("/api/{board_id}", get(get_board_by_id))
        .route("/api/g", post(create_board))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer_restrictions)
        ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:10000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
