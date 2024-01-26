pub mod config;
pub mod counters;

use crate::counters::{
    byte_counter::ByteCounter, char_counter::CharCounter, line_counter::LineCounter,
    word_counter::WordCounter,
};

use crate::config::Config;
use std::fs;
use std::io::{self, Read};
use std::rc::Rc;

pub trait Countable {
    fn len(&self) -> Result<usize, io::Error>;
}

pub trait Source {
    fn read(&self) -> Result<String, io::Error>;

    fn title(&self) -> &str {
        ""
    }
}

struct FileSource {
    file_path: String,
}

impl Source for FileSource {
    fn read(&self) -> Result<String, io::Error> {
        fs::read_to_string(&self.file_path)
    }

    fn title(&self) -> &str {
        &self.file_path
    }
}

struct StdinSource {}

impl Source for StdinSource {
    fn read(&self) -> Result<String, io::Error> {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}

pub struct Wc {
    source: Rc<dyn Source>,
    counters: Vec<Box<dyn Countable>>,
}

impl Wc {
    pub fn new(config: Config) -> Self {
        let config = Self::modify_config_if_needed(config);
        let source = Self::build_source(&config);
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

    fn build_source(config: &Config) -> Rc<dyn Source> {
        match &config.file_path {
            Some(file_path) => Rc::new(FileSource {
                file_path: file_path.clone(),
            }),
            None => Rc::new(StdinSource {}),
        }
    }

    fn build_counters<'a>(
        config: &Config,
        source: &Rc<dyn Source>,
    ) -> Vec<Box<dyn Countable + 'a>> {
        config.flags()
                .into_iter()
                .filter_map(|(flag, value)| {
                    if value {
                        match flag {
                            "count_lines" => {
                                Some(Box::new(LineCounter::new(Rc::clone(source)))
                                    as Box<dyn Countable>)
                            }
                            "count_words" => {
                                Some(Box::new(WordCounter::new(Rc::clone(source)))
                                    as Box<dyn Countable>)
                            }
                            "count_chars" => {
                                Some(Box::new(CharCounter::new(Rc::clone(source)))
                                    as Box<dyn Countable>)
                            }
                            "count_bytes" => {
                                Some(Box::new(ByteCounter::new(Rc::clone(source)))
                                    as Box<dyn Countable>)
                            }
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
