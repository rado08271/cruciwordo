use std::cmp::max;
use types::{Board, Placement};
use crate::board::BoardTrait;
use crate::dictionary::Dictionary;
use spacetimedb::{StdbRng};
use spacetimedb::rand::prelude::SliceRandom;

mod board;
mod dictionary;

pub fn generate_board(rows: usize, cols: usize, message: String, number_rng: &StdbRng) -> Board {
    let mut board: Board = Board::new(rows, cols, message.clone());
    let mut dictionary: Dictionary = Dictionary::from_file("./res/en.dr".to_string(), max(rows, cols));
    let mut placements: Vec<Placement> = Vec::new();

    while !board.is_filled() {
        let random_word = dictionary.get_random_word(number_rng);
        let valid_placement = board.place_word_on_board(random_word, number_rng);

        if valid_placement.is_some() {
            placements.push(valid_placement.unwrap());
        }
    }

    return board;
}

pub fn generate_board_with_dictionary(rows: usize, cols: usize, message: String, words: &Vec<String>, number_rng: &StdbRng) -> (Board, Vec<Placement>) {
    let mut board: Board = Board::new(rows, cols, message.clone());
    let mut dictionary: Dictionary = Dictionary::from_words(words.clone(), max(rows, cols), number_rng);
    let mut placements: Vec<Placement> = Vec::new();

    while !board.is_filled() {
        let random_word = dictionary.get_random_word(number_rng);
        let valid_placement = board.place_word_on_board(random_word, number_rng);

        if valid_placement.is_some() {
            placements.push(valid_placement.unwrap());
        }
    }

    return (board, placements);
}

pub fn generate_random_id(length: usize, mut number_rng: &StdbRng) -> String {
    pub const SAFE: [char; 63] = [
        '_', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
        'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let mut generated_id: String = String::new();

    for _i in 0..length {
        let random_character = SAFE.choose(&mut number_rng);
        if random_character.is_some() {
            generated_id.push(random_character.unwrap().clone());
        }
    }

    return generated_id;
}

#[macro_export]
macro_rules! id {
    ($size:tt, $rng:expr) => {
        $crate::generate_random_id($size, $rng)
    };
}