use std::slice::IterMut;
use rand::prelude::SliceRandom;
use rand::{Rng, thread_rng};

const MIN_WORD_LENGTH: usize = 3;

pub struct Dictionary {
    words: Vec<String>
}

impl Dictionary {
    pub fn from_file(file_path: String, max_word_length: usize) -> Self {
        let items: Vec<String> = std::fs::read_to_string(file_path)
            .unwrap()  // panic on possible file-reading errors
            .lines()  // split the string into an iterator of string slices
            .map(String::from)  // make each slice into a string
            .filter(|s| s.len() >= MIN_WORD_LENGTH && s.len() <= max_word_length)
            .map(|s| s.to_uppercase())
            .collect();  // gather them together into a vector

        Dictionary {
            words: items
                .choose_multiple(&mut thread_rng(), items.len())
                .cloned().collect()
        }
    }

    pub fn iterate(&mut self) -> IterMut<'_, String> {
        return self.words.iter_mut()
    }

    pub fn get_random_word(&mut self) -> String {
        let mut word: Option<&String> = None;

        while word.is_none() {
            let rnd_idx = thread_rng().gen_range(0, self.words.len());
            word = self.words.get(rnd_idx);
        }

        return word.unwrap().to_string();
    }
}