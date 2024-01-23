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
        let flags = Self::set_flags(config.flags);
        let file_path = Rc::new(config.file_path);

        let counters: Vec<Box<dyn Countable>> = flags
            .into_iter()
            .map(|flag| match flag {
                "count_lines" => {
                    Box::new(LineCounter::new(Rc::clone(&file_path))) as Box<dyn Countable>
                }
                "count_words" => {
                    Box::new(WordCounter::new(Rc::clone(&file_path))) as Box<dyn Countable>
                }
                "count_chars" => {
                    Box::new(CharCounter::new(Rc::clone(&file_path))) as Box<dyn Countable>
                }
                "count_bytes" => {
                    Box::new(ByteCounter::new(Rc::clone(&file_path))) as Box<dyn Countable>
                }
                _ => panic!("Wrong counter"),
            })
            .collect();

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

    fn set_flags(mut flags: Vec<&str>) -> Vec<&str> {
        if flags.is_empty() {
            flags = vec!["count_lines", "count_words", "count_bytes"]
        }

        flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_true_counter_results() {
        let config_flags = vec!["count_lines", "count_bytes"];

        let config = Config {
            flags: config_flags,
            file_path: String::from("fixtures/lorem.txt"),
        };

        let wc = Wc::new(config);

        assert_eq!(wc.run(), "3 42 fixtures/lorem.txt")
    }

    #[test]
    fn return_line_word_bytes_counter_results_if_no_flags_set() {
        let config_flags: Vec<&str> = Vec::new();
        let config = Config {
            flags: config_flags,
            file_path: String::from("fixtures/lorem.txt"),
        };

        let wc = Wc::new(config);

        assert_eq!(wc.run(), "3 6 42 fixtures/lorem.txt")
    }
}
