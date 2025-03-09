use std::collections::HashSet;
use db::{PuzzleDAO, PuzzleDTO};
use types::Board;

pub fn puzzle_dto_to_board(puzzle: PuzzleDTO) -> Board {
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
        rows: puzzle.rows, cols: puzzle.cols,
        solution: puzzle.solution, message: puzzle.original_message,
        grid, words
    };
}

pub fn puzzle_dto_to_puzzle_dao(puzzle_dto: PuzzleDTO) -> PuzzleDAO {
    return PuzzleDAO {
        rows: puzzle_dto.rows as u64, cols: puzzle_dto.cols as u64,
        solution: puzzle_dto.solution, original_message: puzzle_dto.original_message,
        grid: puzzle_dto.grid, history: puzzle_dto.history, id: puzzle_dto.id, created_date: puzzle_dto.created_date
    };
}
