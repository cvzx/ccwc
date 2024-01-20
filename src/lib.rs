pub mod config;
pub mod counters;

use crate::config::Config;
use crate::counters::{
    byte_counter::ByteCounter, char_counter::CharCounter, line_counter::LineCounter,
    word_counter::WordCounter,
};
use std::error::Error;

pub trait Countable {
    fn len(&self) -> Result<usize, Box<dyn Error>>;
}

struct Ccwc {
    counters: Vec<Box<dyn Countable>>,
}

impl Ccwc {
    pub fn new(config: Config) -> Self {
        let mut counters: Vec<Box<dyn Countable>> = Vec::new();

        if config.count_bytes == true {
            counters.push(Box::new(ByteCounter::new(&config.file_path)));
        }

        if config.count_lines == true {
            counters.push(Box::new(LineCounter::new(&config.file_path)));
        }

        if config.count_words == true {
            counters.push(Box::new(WordCounter::new(&config.file_path)));
        }

        if config.count_chars == true {
            counters.push(Box::new(CharCounter::new(&config.file_path)));
        }

        Self { counters }
    }

    pub fn run(&self) -> String {
        // let counts = self.counters.iter().map(|counter| counter.len()).collect();

        "test".to_string()
    }
}
