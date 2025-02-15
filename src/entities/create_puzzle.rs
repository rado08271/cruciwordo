use std::cmp::max;
use axum::{extract, Json};
use axum::http::StatusCode;
use chrono::Utc;
use nanoid::nanoid;
use serde::Deserialize;
use skytable::{Config, query};
use crate::board::{Board, Placement};
use crate::dictionary::Dictionary;
use crate::entities::get_database;
use crate::entities::get_puzzle::PuzzleDAO;

#[derive(Deserialize)]
pub struct Params {
    rows: usize,
    cols: usize,
    message: String
}

pub async fn generate_new_board (
    extract::Json(
        Params {rows, cols, message}
    ): extract::Json<Params>
) -> (StatusCode, Json<String>) {

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

    let grid_string: String = board.grid.iter()
        .map(|r| r.iter().map(|c| format!("{}", c)).collect::<String>())
        .rfold(String::from(""), |data, r| format!("{}{}", r, data));

    let history_sequences = placements.iter().map(|p|
        format!("{}|{}|{}|{}|{}", p.step, p.word,p.row,p.col,p.direction)
    ).collect::<Vec<String>>();

    let board_id = nanoid!(10, &nanoid::alphabet::SAFE);
    let result_board: PuzzleDAO = PuzzleDAO {
        id: board_id.clone(),
        created_date: Utc::now().to_rfc3339(),
        original_message: message.clone(),
        solution: board.solution,
        grid: grid_string,
        history: history_sequences,
        rows: rows as u64, cols: cols  as u64,
    };

    let mut db = get_database();

    let mut insert_query = query!(
        "insert into cruciwordo.puzzle ( ?, ?, ?, ?, ?, ?, ?, ? )",
        &result_board
    );

    db.unwrap().query_parse::<()>(&insert_query).unwrap();

    return (
        StatusCode::OK,
        Json((board_id.clone()))
    );
}
