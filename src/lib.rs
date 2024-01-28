pub mod config;
pub mod counters;

use crate::config::Config;
use crate::counters::Counter;
use std::io::Read;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub struct Wc {
    source_name: Option<String>,
    counters: Vec<Counter>,
}

impl Wc {
    pub fn new(input: impl Read, config: Config) -> Self {
        let config = Self::modify_config_if_needed(config);
        let content = Self::read_content(input);
        let counters = Self::build_counters(&config, Arc::new(content));
        let source_name = config.file_path;

        Self {
            source_name,
            counters,
        }
    }

    pub fn count(self) -> String {
        let (tx, rx) = mpsc::channel();

        self.counters
            .into_iter()
            .for_each(|counter| Self::run_counter(tx.clone(), counter));

        drop(tx);

        let counts = rx.into_iter().collect::<Vec<String>>();

        match &self.source_name {
            Some(name) => format!(" {}  {}", counts.join("  "), name),
            None => format!(" {}", counts.join("  ")),
        }
    }

    fn modify_config_if_needed(config: Config) -> Config {
        if config.flags().iter().all(|(_, value)| !value) {
            Config {
                count_lines: true,
                count_words: true,
                count_bytes: true,
                ..config
            }
        } else {
            config
        }
    }

    fn read_content(mut input: impl Read) -> String {
        let mut content = String::new();

        input
            .read_to_string(&mut content)
            .expect("Error reading content");

        content
    }

    fn build_counters(config: &Config, content: Arc<String>) -> Vec<Counter> {
        config
            .flags()
            .into_iter()
            .filter_map(|(flag, value)| {
                if value {
                    match flag {
                        "count_lines" => Some(Counter::new("Lines", Arc::clone(&content))),
                        "count_words" => Some(Counter::new("Words", Arc::clone(&content))),
                        "count_chars" => Some(Counter::new("Chars", Arc::clone(&content))),
                        "count_bytes" => Some(Counter::new("Bytes", Arc::clone(&content))),
                        _ => panic!("Wrong counter"),
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn run_counter(tx: mpsc::Sender<String>, counter: Counter) {
        thread::spawn(move || {
            tx.send(counter.len().to_string()).expect("Counting error");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_file_counts_accoring_to_flags_privided() {
        let input = b"test test\ntest1 test1\ntest2 test2";

        let config = Config {
            count_lines: true,
            count_words: true,
            count_chars: false,
            count_bytes: false,
            file_path: Some(String::from("fixtures/lorem.txt")),
        };

        let wc = Wc::new(&input[..], config);

        assert_eq!(wc.count(), " 3  6  fixtures/lorem.txt")
    }

    #[test]
    fn return_line_word_bytes_counter_results_if_no_flags_set() {
        let input = b"test test\ntest1 test1\ntest2 test2";

        let config = Config {
            count_lines: false,
            count_words: false,
            count_chars: false,
            count_bytes: false,
            // stdin case
            file_path: None,
        };
        let wc = Wc::new(&input[..], config);

        assert_eq!(wc.count(), " 3  6  33")
    }
}
