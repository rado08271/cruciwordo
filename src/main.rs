mod board;
mod dictionary;

use std::cmp::max;
use crate::board::{Board, Placement};
use crate::dictionary::Dictionary;

fn main() {
    let rows = 5;
    let cols = 5;
    let message = "This is long?".to_string();

    let mut board: Board = Board::new(rows, cols, message);
    let mut dictionary: Dictionary = Dictionary::from_file("./res/en.dr".to_string(), max(rows, cols));
    let mut placements: Vec<Placement> = Vec::new();

    while !board.is_filled() {
        let random_word = dictionary.get_random_word();
        let valid_placement = board.place_word_on_board(random_word);

        if valid_placement.is_some() {
            placements.push(valid_placement.unwrap());
        }
    }

    board.print();
    placements.iter().for_each(|p| println!("{}. word {} at R{}C{} in D:{}", p.step, p.word, p.row, p.col, p.direction));

    println!("🏆 Board is now ready");
}
