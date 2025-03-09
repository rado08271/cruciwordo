use std::collections::HashSet;

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub message: String,
    pub solution: String,
    pub grid: Vec<Vec<char>>,
    pub words: HashSet<String>
}

pub struct Placement {
    pub direction: &'static str,
    pub row: usize,
    pub col: usize,
    pub word: String,
    pub step: usize
}

#[derive(Copy, Clone)]
pub struct Direction {
    pub x_dir: isize,
    pub y_dir: isize,
    pub dbg_name: &'static str
}