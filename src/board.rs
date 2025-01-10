use std::collections::HashSet;
use rand::prelude::SliceRandom;
use rand::{Rng, thread_rng};

#[derive(Clone, Copy)]
pub struct Direction {
    pub(crate) x_dir: isize,
    pub(crate) y_dir: isize
}

const DIRECTIONS: [Direction; 8] = [
    Direction {y_dir: -1, x_dir: -1},   // NW
    Direction {y_dir: -1, x_dir:  0},   // N
    Direction {y_dir: -1, x_dir:  1},   // NE
    Direction {y_dir:  0, x_dir: -1},   // W
    Direction {y_dir:  0, x_dir:  1},   // E
    Direction {y_dir:  1, x_dir: -1},   // SW
    Direction {y_dir:  1, x_dir:  0},   // S
    Direction {y_dir:  1, x_dir:  1},   // SE
];

pub struct Board {
    rows: usize,
    cols: usize,
    message: String,
    solution: String,
    grid: Vec<Vec<char>>,
    words: HashSet<String>
}

impl Board {
    pub fn new(rows: usize, cols: usize, message: String) -> Self {
        let solution = message.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
        Board {
            cols, rows,
            message, solution,
            grid: vec![vec!['?'; cols]; rows],
            words: HashSet::new()
        }
    }

    pub fn is_filled(&self) -> bool {
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

    pub fn has_enough_empty_cells(&self) -> bool {
        let mut items: usize = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == '?' {
                    items += 1;
                }

                if items >= self.solution.len() {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn place_word_on_board(&mut self, row: usize, col: usize, direction: Direction, word: String) {
        for curr_depth in 0..word.len() {
            let curr_char = word.chars().nth(curr_depth).unwrap_or(' ');

            let irow_depth: isize = (row as isize + (direction.y_dir * curr_depth as isize));
            let icol_depth: isize = (col as isize + (direction.x_dir * curr_depth as isize));

            if irow_depth < 0 || icol_depth < 0 {
                return;
            }

            let row_depth: usize = irow_depth as usize;
            let col_depth: usize = icol_depth as usize;

            if row_depth >= self.rows || col_depth >= self.cols {
                return;
            }

            self.grid[row_depth][col_depth] = curr_char;
        }

        self.words.insert(word);
    }

    pub fn word_fits_board(&self, row: usize, col: usize, direction: Direction, word: String) -> bool {
        for curr_depth in 0..word.len() {
            let curr_char = word.chars().nth(curr_depth).unwrap_or(' ');

            let irow_depth: isize = (row as isize + (direction.y_dir * curr_depth as isize));
            let icol_depth: isize = (col as isize + (direction.x_dir * curr_depth as isize));

            if irow_depth < 0 || icol_depth < 0 {
                return false;
            }

            let row_depth: usize = irow_depth as usize;
            let col_depth: usize = icol_depth as usize;

            if row_depth >= self.rows || col_depth >= self.cols {
                return false;
            }


            let cell_char = self.grid[row_depth][col_depth];

            if cell_char != '?' && curr_char != cell_char {
                return false
            }

            if !self.has_enough_empty_cells() {
                return false;
            }

        }

        return true;

    }

    pub fn get_random_cell(&self) -> (usize, usize) {
        // make it start at random
        let mut row = thread_rng().gen_range(0, self.rows);
        let mut col = thread_rng().gen_range(0, self.cols);

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

    pub fn get_random_direction_from_cell(&self, row: usize, col: usize, depth: usize) -> Option<Direction> {
        let random_directions: Vec<Direction> = DIRECTIONS
            .choose_multiple(&mut thread_rng(), DIRECTIONS.len())
            .map(|d| *d)
            .collect();

        for direction in random_directions {
            for curr_depth in 0..depth {
                let irow_depth: isize = (row as isize + (direction.y_dir * curr_depth as isize));
                let icol_depth: isize = (col as isize + (direction.x_dir * curr_depth as isize));

                if irow_depth < 0 || icol_depth < 0 {
                    break;
                }

                let row_depth: usize = irow_depth as usize;
                let col_depth: usize = icol_depth as usize;

                if row_depth >= self.rows || col_depth >= self.cols {
                    break;
                }

            }
            return Some(direction);
        }

        return None;
    }

    pub fn print(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let cell_char = self.grid[r][c];
                print!("[{}{}] {}\t", r, c, cell_char);
            }
            println!();
        }

        self.words.iter().for_each(|w| println!("\t{}", w));
        println!("S: {}", self.solution);
    }
}
