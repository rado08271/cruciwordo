mod board;
mod dictionary;

use std::cmp::max;
use crate::board::Board;
use crate::dictionary::Dictionary;

fn main() {
    let rows = 5;
    let cols = 5;
    let message = "This is long?".to_string();

    let mut board: Board = Board::new(rows, cols, message);
    let mut dictionary: Dictionary = Dictionary::from_file("./res/en.dr".to_string(), max(rows, cols));

    for word in dictionary.iterate() {
        board.place_word_on_board(word.clone());

        if board.is_filled() {
            board.print();
            println!("🏆 Board is now ready");
            return;
        }
    }

    board.print();
    println!("😭 Better luck next time!");
}
