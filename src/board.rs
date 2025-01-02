use std::collections::HashSet;

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
        // The preprocessing of an input removes special characters, whitespaces and numeric characters
        let solution: String = message.to_uppercase().chars().filter(|c| c.is_alphabetic()).collect();

        Board {
            cols, rows,
            message, solution,
            grid: vec![vec!['?'; cols]; rows],
            words: HashSet::new()
        }
    }
}
