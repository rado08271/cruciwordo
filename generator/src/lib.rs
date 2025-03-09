use std::cmp::max;
use types::{Board, Placement};
use crate::dictionary::Dictionary;

mod board;
mod dictionary;

pub fn generate_board(rows: usize, cols: usize, message: String) -> Board {
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

    return board;
}
