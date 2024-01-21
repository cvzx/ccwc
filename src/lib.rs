pub mod config;
pub mod counters;

use crate::config::Config;
use crate::counters::{
    byte_counter::ByteCounter, char_counter::CharCounter, line_counter::LineCounter,
    word_counter::WordCounter,
};
use std::error::Error;
use std::rc::Rc;

pub trait Countable {
    fn len(&self) -> Result<usize, Box<dyn Error>>;
}

pub struct Wc {
    file_path: String,
    counters: Vec<Box<dyn Countable>>,
}

impl Wc {
    pub fn new(config: Config) -> Self {
        let mut counters: Vec<Box<dyn Countable>> = Vec::new();
        let file_path = Rc::new(config.file_path);

        if config.count_lines == true {
            counters.push(Box::new(LineCounter::new(Rc::clone(&file_path))));
        }

        if config.count_words == true {
            counters.push(Box::new(WordCounter::new(Rc::clone(&file_path))));
        }

        if config.count_chars == true {
            counters.push(Box::new(CharCounter::new(Rc::clone(&file_path))));
        }

        if config.count_bytes == true {
            counters.push(Box::new(ByteCounter::new(Rc::clone(&file_path))));
        }

        Self {
            counters,
            file_path: file_path.to_string(),
        }
    }

    pub fn run(&self) -> String {
        let counts: Vec<String> = self
            .counters
            .iter()
            .map(|counter| counter.len().expect("Counting error").to_string())
            .collect();

        format!("{} {}", counts.join(" "), &self.file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_all_counter_results() {
        let config = Config {
            count_bytes: true,
            count_lines: true,
            count_words: true,
            count_chars: true,
            count_all: false,
            file_path: String::from("fixtures/lorem.txt"),
        };

        let wc = Wc::new(config);

        assert_eq!(wc.run(), "3 6 42 42 fixtures/lorem.txt")
    }
}
