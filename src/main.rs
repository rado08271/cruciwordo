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

    while !board.is_filled() {
        let random_word = dictionary.get_random_word();
        board.place_word_on_board(random_word);
    }

    board.print();
    println!("🏆 Board is now ready");
}
