mod puzzle_dao;

use serde::Serialize;
use skytable::{ClientResult, Config, Connection, Pipeline, query};
use types::Board;
use crate::puzzle_dao::{board_to_puzzle_dao,  puzzle_dao_to_board, puzzle_dao_to_puzzle_dto, PuzzleDAO};

pub fn initialize_db() {

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

pub fn get_database () -> ClientResult<Connection> {
    let skytable_database_root_user: String = std::env::var("SKYTABLE_DATABASE_ROOT_USER").expect("SKYTABLE_DATABASE_ROOT_USER must be set");
    let skytable_database_root_pass: String = std::env::var("SKYTABLE_DATABASE_ROOT_PASS").expect("SKYTABLE_DATABASE_ROOT_PASS must be set");
    let skytable_database_root_host: String = std::env::var("SKYTABLE_DATABASE_ROOT_HOST").expect("SKYTABLE_DATABASE_ROOT_HOST must be set");
    let skytable_database_root_port: u16  = std::env::var("SKYTABLE_DATABASE_ROOT_PORT").expect("SKYTABLE_DATABASE_ROOT_PORT must be set").parse::<u16>().expect("Cannot parse SKYTABLE_DATABASE_ROOT_PORT to number");

    println!("s://{}:{}@{}:{}",skytable_database_root_user, skytable_database_root_pass, skytable_database_root_host, skytable_database_root_port);

    let mut db = Config::new(
        skytable_database_root_host.as_str(),
        skytable_database_root_port,
        skytable_database_root_user.as_str(),
        skytable_database_root_pass.as_str()
    ).connect();

    return db;
}

pub fn insert_puzzle(board: Board) -> Result<String, String> {
    let puzzle_dao: PuzzleDAO = board_to_puzzle_dao(board);

    let mut db = get_database();

    let mut insert_query = query!(
        "insert into cruciwordo.puzzle ( ?, ?, ?, ?, ?, ?, ?, ? )",
        &puzzle_dao
    );

    let mut insert_puzzle_status = db.unwrap().query_parse::<()>(&insert_query);

    if insert_puzzle_status.is_ok() {
        return Ok(puzzle_dao.id)
    }

    return Err("Problem here".to_string())
}

pub fn get_puzzle_by_id(puzzle_id: String) -> Result<PuzzleDTO, String> {
    let mut db = get_database();
    let mut select_query = query!(
        "select * from cruciwordo.puzzle where id = ?", puzzle_id.as_str()
    );

    let mut select_puzzle_status = db.unwrap().query_parse::<PuzzleDAO>(&select_query);

    if select_puzzle_status.is_ok() {
        let puzzle_dao = select_puzzle_status.unwrap();

        return Ok(puzzle_dao_to_puzzle_dto(puzzle_dao));
    }

    return Err("Problem here".to_string())
}

#[derive(Serialize, Clone)]
pub struct PuzzleDAO {
    pub id: String,
    pub created_date: String,
    pub rows: u64,
    pub cols: u64,
    pub original_message: String,
    pub solution: String,
    pub grid: String,
    pub history: Vec<String>
}

#[derive(Serialize)]
pub struct PuzzleDTO {
    pub id: String,
    pub created_date: String,
    pub rows: usize,
    pub cols: usize,
    pub original_message: String,
    pub solution: String,
    pub grid: String,
    pub history: Vec<String>
}
