use axum::{extract, Json};
use axum::http::StatusCode;
use serde::Serialize;
use skytable::{ClientResult, Config, query};
use skytable::query::{QList, SQParam};
use skytable::response::{FromResponse, RList};
use crate::entities::get_database;

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

impl SQParam for PuzzleDAO {
    fn append_param(&self, q: &mut Vec<u8>) -> usize {
        self.id.append_param(q) +
            self.created_date.append_param(q) +
            self.rows.append_param(q) +
            self.cols.append_param(q) +
            self.original_message.append_param(q) +
            self.solution.append_param(q) +
            self.grid.append_param(q) +
            QList::new(&self.history).append_param(q)
    }
}

impl FromResponse for PuzzleDAO {
    fn from_response(resp: skytable::response::Response) -> ClientResult<Self> {
        let (id, created_date, rows, cols, original_message, solution, grid, history) = resp.parse::<(String, String, u64, u64, String, String, String, RList<String>)>()?;
        Ok (PuzzleDAO {
            id, created_date, rows, cols, original_message, solution, grid, history: history.into_values()
        })
    }
}

pub async fn get_board_by_id (
    extract::Path(board_id): extract::Path<String>
) -> Result<Json<PuzzleDTO>, (StatusCode, String)> {
    let mut db = get_database();
    let mut select_query = query!(
        "select * from cruciwordo.puzzle where id = ?", board_id.as_str()
    );

    let mut puzzle_dao_wrapped = db.unwrap().query_parse::<PuzzleDAO>(&select_query);

    if puzzle_dao_wrapped.is_err() {
        let error = puzzle_dao_wrapped.err();

        if error.is_some() {
            return axum::response::Result::Err ((
                StatusCode::BAD_REQUEST,
                error.unwrap().to_string()
            ));
        }

        return axum::response::Result::Err ((
            StatusCode::BAD_REQUEST,
            format!("There was an error while processing")
        ));
    }

    let mut puzzle_dao = puzzle_dao_wrapped.unwrap();

    let puzzle_dto = PuzzleDTO {
        history: puzzle_dao.history,
        id: puzzle_dao.id,
        solution: puzzle_dao.solution,
        created_date: puzzle_dao.created_date,
        rows: puzzle_dao.rows as usize, cols: puzzle_dao.cols as usize,
        original_message: puzzle_dao.original_message,
        grid: puzzle_dao.grid
    };

    return Ok(
        Json(puzzle_dto)
    );
}
