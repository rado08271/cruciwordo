use std::collections::HashSet;
use chrono::Utc;
use nanoid::nanoid;
use serde::Serialize;
use skytable::ClientResult;
use skytable::query::{QList, SQParam};
use skytable::response::FromResponse;
use types::Board;
use crate::{PuzzleDAO, PuzzleDTO};

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

pub fn board_to_puzzle_dao(board: Board) -> PuzzleDAO {
    let board_id = nanoid!(10, &nanoid::alphabet::SAFE);

    let grid_string: String = board.grid.iter()
        .map(|r| r.iter().map(|c| format!("{}", c)).collect::<String>())
        .rfold(String::from(""), |data, r| format!("{}{}", r, data));

    let history: Vec<String> = board.words.iter().map(|w|w).collect();

    let result_puzzle: PuzzleDAO = PuzzleDAO {
        id: board_id.clone(),
        created_date: Utc::now().to_rfc3339(),
        original_message: board.message.clone(),
        solution: board.solution,
        grid: grid_string,
        rows: board.rows as u64, cols: board.cols as u64,
        history,
    };

    return result_puzzle;
}

pub fn puzzle_dao_to_board(puzzle: PuzzleDAO) -> Board {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut peeked = puzzle.grid.chars().peekable();

    while peeked.peek().is_some() {
        let row: String = peeked.by_ref().take(puzzle.rows as usize).collect();
        let cols: Vec<char> = row.chars().map(|c|c).collect();

        grid.push(cols)
    }

    let mut words = HashSet::new();
    for word in puzzle.history {
        words.insert(word);
    }

    return Board {
        rows: puzzle.rows as usize, cols: puzzle.cols as usize,
        grid: grid, solution: puzzle.solution, message: puzzle.original_message,
        words
    }
}

pub fn puzzle_dao_to_puzzle_dto(puzzle_dao: PuzzleDAO) -> PuzzleDTO {
    return PuzzleDTO {
        id: puzzle_dao.id,
        solution: puzzle_dao.solution,
        created_date: puzzle_dao.created_date,
        rows: puzzle_dao.rows as usize, cols: puzzle_dao.cols as usize,
        original_message: puzzle_dao.original_message,
        grid: puzzle_dao.grid,
        history: puzzle_dao.history,
    };

}