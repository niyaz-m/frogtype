use rand::seq::SliceRandom;
use std::fs;

pub struct WordBank {
    words: Vec<String>,
}

impl WordBank {
    pub fn init() -> Self {
        let content = fs::read_to_string("assests/english_200.txt")
            .expect("Failed to find words in assests/english_200.txt");

        let words: Vec<String> = content
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        WordBank { words }
    }

    pub fn get_random_word(&self, length: usize) -> String {
        let mut rng = rand::thread_rng();

        self.words
            .choose_multiple(&mut rng, length)
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
