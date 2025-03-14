use types::{Board, Direction, Placement};

use std::collections::HashSet;
use spacetimedb::rand::prelude::SliceRandom;
use spacetimedb::{rand::Rng, StdbRng};

const DIRECTIONS: [Direction; 8] = [
    Direction { y_dir: -1, x_dir: -1, dbg_name: "NW" },   // NW
    Direction { y_dir: -1, x_dir: 0, dbg_name: "N" },   // N
    Direction { y_dir: -1, x_dir: 1, dbg_name: "NE" },   // NE
    Direction { y_dir: 0, x_dir: -1, dbg_name: "W" },   // W
    Direction { y_dir: 0, x_dir: 1, dbg_name: "E" },   // E
    Direction { y_dir: 1, x_dir: -1, dbg_name: "SW" },   // SW
    Direction { y_dir: 1, x_dir: 0, dbg_name: "S" },   // S
    Direction { y_dir: 1, x_dir: 1, dbg_name: "SE" },   // SE
];

pub trait BoardTrait {
    fn new(rows: usize, cols: usize, message: String) -> Board;
    fn is_filled(&self) -> bool;
    fn get_empty_cells(&self) -> usize;
    fn place_word_on_board(&mut self, word: String, rng: &StdbRng) -> Option<Placement>;
    fn get_random_cell(&self, rng: &StdbRng) -> (usize, usize);
    fn word_fits_board_direction(&self, row: usize, col: usize, word: String, rng: &StdbRng) -> Option<Direction>;
}

impl BoardTrait for Board {
    fn new(rows: usize, cols: usize, message: String) -> Self {
        let solution = message.to_uppercase().chars().filter(|c| c.is_alphabetic()).collect();
        Board {
            cols, rows,
            message, solution,
            grid: vec![vec!['?'; cols]; rows],
            words: HashSet::new()
        }
    }

    fn is_filled(&self) -> bool {
        let mut items: usize = 0;

        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == '?' {
                    items += 1;
                }
            }
        }

        return items == self.solution.len();
    }

    fn get_empty_cells(&self) -> usize {
        let mut items: usize = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == '?' {
                    items += 1;
                }
            }
        }

        return items;
    }

    fn place_word_on_board(&mut self, word: String, rng: &StdbRng) -> Option<Placement> {
        let (row, col) = self.get_random_cell(rng);
        // FIXME: We should ensure that word is not part of solution - this may be problem if solution 'stable' will be placed in allowed direction fully or partially and words like 'table, able' might fit there
        let o_direction = self.word_fits_board_direction(row, col, word.clone(), rng);

        if o_direction.is_some() {
            let direction = o_direction.unwrap();

            for (curr_depth, curr_char) in word.chars().enumerate() {
                let irow_depth: isize = row as isize + (direction.y_dir * curr_depth as isize);
                let icol_depth: isize = col as isize + (direction.x_dir * curr_depth as isize);

                self.grid[irow_depth as usize][icol_depth as usize] = curr_char;
            }

            self.words.insert(word.clone());

            return Some( Placement {
                word, row, col, direction: direction.dbg_name, step: self.words.len()
            });
        }

        return None;
    }

    fn get_random_cell(&self, mut rng: &StdbRng) -> (usize, usize) {
        // make it start at random
        // TODO : Update RND
        let mut row = rng.gen_range(0..self.rows);
        let mut col = rng.gen_range(0..self.cols);

        while self.grid[row][col] != '?' {
            col += 1;
            if col == self.cols {
                col = 0;
                // modulo to ensure it won't access outside range
                row = (row + 1) % self.rows;
            }
        }

        (row, col)
    }

    fn word_fits_board_direction(&self, row: usize, col: usize, word: String, mut rng: &StdbRng) -> Option<Direction> {
        if self.words.contains(&word) {
            return None
        }

        // if (row + word.len() > self.rows) || (col + word.len() > self.cols) {
        //     return None;
        // }

        let mut random_directions = DIRECTIONS.clone();
        // TODO : Update RND
        random_directions.shuffle(&mut rng);

        for direction in random_directions {
            let mut non_filled_cells: usize = 0;
            for (curr_depth, curr_char) in word.chars().enumerate() {
                let irow_depth: isize = row as isize + (direction.y_dir * curr_depth as isize);
                let icol_depth: isize = col as isize + (direction.x_dir * curr_depth as isize);

                if irow_depth < 0 || icol_depth < 0 {
                    break;
                }

                let row_depth: usize = irow_depth as usize;
                let col_depth: usize = icol_depth as usize;

                if row_depth >= self.rows || col_depth >= self.cols {
                    break;
                }

                let cell_char = self.grid[row_depth][col_depth];

                if cell_char != '?' && curr_char != cell_char {
                    break;
                }

                if cell_char == '?' {
                    non_filled_cells += 1;
                }

                if curr_depth == word.len() - 1 {
                    if (self.solution.len() + non_filled_cells) > self.get_empty_cells() {
                        break;
                    }

                    return Some(direction);
                }
            }
        }

        return None;
    }
}
