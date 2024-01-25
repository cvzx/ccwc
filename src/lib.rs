pub mod config;
pub mod counters;

use crate::counters::{
    byte_counter::ByteCounter, char_counter::CharCounter, line_counter::LineCounter,
    word_counter::WordCounter,
};
use clap::ArgMatches;
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
    pub fn new(matches: ArgMatches) -> Self {
        let source = Self::build_source(&matches);
        let counters = Self::build_counters(&matches, &source);

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

    fn build_source(matches: &ArgMatches) -> Rc<dyn Source> {
        match matches.get_one::<String>("file_path") {
            Some(x) => Rc::new(FileSource {
                file_path: x.to_string(),
            }),
            None => Rc::new(StdinSource {}),
        }
    }

    fn build_counters<'a>(
        matches: &ArgMatches,
        source: &Rc<dyn Source>,
    ) -> Vec<Box<dyn Countable + 'a>> {
        let mut flags: Vec<&str> = vec!["count_lines", "count_words", "count_chars", "count_bytes"]
            .into_iter()
            .filter(|&flag| matches.get_flag(flag))
            .collect();

        if flags.is_empty() {
            flags = vec!["count_lines", "count_words", "count_bytes"]
        }

        flags
            .into_iter()
            .map(|flag| match flag {
                "count_lines" => {
                    Box::new(LineCounter::new(Rc::clone(source))) as Box<dyn Countable>
                }
                "count_words" => {
                    Box::new(WordCounter::new(Rc::clone(source))) as Box<dyn Countable>
                }
                "count_chars" => {
                    Box::new(CharCounter::new(Rc::clone(source))) as Box<dyn Countable>
                }
                "count_bytes" => {
                    Box::new(ByteCounter::new(Rc::clone(source))) as Box<dyn Countable>
                }
                _ => panic!("Wrong counter"),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, ArgAction, Command};

    #[test]
    fn return_file_counts_accoring_to_flags_privided() {
        let matches = setup_mock_clap_matches(vec!["my_prog", "-l", "-c", "fixtures/lorem.txt"]);
        let wc = Wc::new(matches);

        assert_eq!(wc.count(), "3 42 fixtures/lorem.txt")
    }

    #[test]
    fn return_line_word_bytes_counter_results_if_no_flags_set() {
        let matches = setup_mock_clap_matches(vec!["my_prog", "fixtures/lorem.txt"]);
        let wc = Wc::new(matches);

        assert_eq!(wc.count(), "3 6 42 fixtures/lorem.txt")
    }

    // TODO:  Implement stdin case

    fn setup_mock_clap_matches(args: Vec<&str>) -> ArgMatches {
        Command::new("my_prog")
            .arg(
                Arg::new("count_bytes")
                    .short('c')
                    .long("bytes")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("count_lines")
                    .short('l')
                    .long("lines")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("count_words")
                    .short('w')
                    .long("words")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("count_chars")
                    .short('m')
                    .long("chars")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("count_all")
                    .short('a')
                    .long("all")
                    .action(ArgAction::SetTrue),
            )
            .arg(Arg::new("file_path"))
            .get_matches_from(args)
    }
}
