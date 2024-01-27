pub mod config;
pub mod counters;
pub mod sources;

use crate::config::Config;
use crate::counters::Counter;
use crate::sources::Source;
use std::rc::Rc;

pub struct Wc {
    source: Rc<Source>,
    counters: Vec<Counter>,
}

impl Wc {
    pub fn new(config: Config) -> Self {
        let config = Self::modify_config_if_needed(config);
        let source = Rc::new(Self::build_source(&config));
        let counters = Self::build_counters(&config, &source);

        Self { source, counters }
    }

    pub fn count(&self) -> String {
        let counts: Vec<String> = self
            .counters
            .iter()
            .map(|counter| counter.len().expect("Counting error").to_string())
            .collect();

        format!("{} {}", counts.join(" "), self.source.title())
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

    fn build_source(config: &Config) -> Source {
        match &config.file_path {
            Some(file_path) => Source::File(file_path.to_string()),
            None => Source::StdIn,
        }
    }

    fn build_counters(config: &Config, source: &Rc<Source>) -> Vec<Counter> {
        config
            .flags()
            .into_iter()
            .filter_map(|(flag, value)| {
                if value {
                    match flag {
                        "count_lines" => Some(Counter::new("Lines", Rc::clone(source))),
                        "count_words" => Some(Counter::new("Words", Rc::clone(source))),
                        "count_chars" => Some(Counter::new("Chars", Rc::clone(source))),
                        "count_bytes" => Some(Counter::new("Bytes", Rc::clone(source))),
                        _ => panic!("Wrong counter"),
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_file_counts_accoring_to_flags_privided() {
        let config = Config {
            count_lines: true,
            count_words: true,
            count_chars: false,
            count_bytes: false,
            file_path: Some(String::from("fixtures/lorem.txt")),
        };

        let wc = Wc::new(config);

        assert_eq!(wc.count(), "3 6 fixtures/lorem.txt")
    }

    #[test]
    fn return_line_word_bytes_counter_results_if_no_flags_set() {
        let config = Config {
            count_lines: false,
            count_words: false,
            count_chars: false,
            count_bytes: false,
            file_path: Some(String::from("fixtures/lorem.txt")),
        };
        let wc = Wc::new(config);

        assert_eq!(wc.count(), "3 6 42 fixtures/lorem.txt")
    }

    // TODO:  Implement stdin case
}
