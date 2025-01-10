mod board;
mod dictionary;

use std::cmp::max;
use crate::board::Board;
use crate::dictionary::Dictionary;

fn main() {
    /*
        Phase 1 - Prepare board
        - We will select a final solution - a sentence or a word consisting of alpha values
        - We will check whether the solution fits onto the board
        - For 5x5 board and we for sure know that the solution cannot by more than 25 characters
        - We will create an empty board defined by specified number of ROWSxCOLS
        - We read a list of all available words - dictionary
        Phase 2 - Prepare cell
        - We select random not yet used word from shuffled dictionary
        - We get random cell at the board that is empty - for now only empty starting cells - we might enhance with any
        - We iterate directions matrix to find first random direction that does not route outside our board grid
        Phase 3 - Put word
        - We try to put selected word on board in selected direction
        - Word cannot go outside grid
        - Word character cannot replace existing character on board
        - Word after placed cannot fill cells that are designated (empty) for solution
        Phase 4 - Game loop
        - We put a random word on a board
        - If there are more than solution characters empty cells we will continue
        - If word was not put on board (no more suitable available words)
     */

    let rows = 30;
    let cols = 30;
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
                break;
            }
        }
    }

    board.print();
}
