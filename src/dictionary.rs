use rand::prelude::SliceRandom;
use rand::thread_rng;

const MIN_WORD_LENGTH: usize = 3;

pub struct Dictionary {
    words: Vec<String>
}

impl Dictionary {
    pub fn from_file(file_path: String, max_word_length: usize) -> Self {
        let items: Vec<String> = std::fs::read_to_string(file_path)
            .unwrap()  // panic on possible file-reading errors
            .lines()  // split the string into an iterator of string slices
            .map(|word| word.split("\t").nth(0).unwrap())
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

    pub fn get_word_at(&self, idx: usize) -> String {
        let word = self.words.get(idx);

        if word.is_some() {
            return String::from(word.unwrap())
        }

        return String::from("");
    }
}