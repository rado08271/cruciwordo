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
        let (row, col) = board.get_random_cell();
        let direction = board.get_random_direction_from_cell(row, col, word.len());

        if direction.is_some() {
            let can_be_placed = board.word_fits_board(row, col, direction.unwrap(), word.clone());

            if can_be_placed {
                println!("✅ Word {} will fit on board at r{}c{} in direction [{};{}]", word, row, col, direction.unwrap().y_dir, direction.unwrap().x_dir);
                board.place_word_on_board(row, col, direction.unwrap(), word.clone());
            } else {
                println!("❌ Word {} will not fit on board at r{}c{} in direction [{};{}]", word, row, col, direction.unwrap().y_dir, direction.unwrap().x_dir);
            }

            if board.is_filled() {
                board.print();
                println!("🏆 Board is now ready");
                return;
            }
        }
    }

    board.print();
    println!("😭 Better luck next time!");
}
