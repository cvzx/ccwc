use crate::config::Config;
use crate::counters::{
    all_counter::AllCounter, byte_counter::ByteCounter, char_counter::CharCounter,
    line_counter::LineCounter, word_counter::WordCounter,
};
use std::error::Error;

pub trait Countable {
    fn len(&self) -> Result<String, Box<dyn Error>>;
}

pub enum Counter<'a> {
    Byte(ByteCounter<'a>),
    Line(LineCounter<'a>),
    Word(WordCounter<'a>),
    Char(CharCounter<'a>),
    All(AllCounter<'a>),
}

impl<'a> Counter<'a> {
    pub fn new(config: &'a Config) -> Result<Self, Box<dyn Error>> {
        match config.counter_type.as_str() {
            "-c" => Ok(Self::Byte(ByteCounter::new(&config.file_path))),
            "-l" => Ok(Self::Line(LineCounter::new(&config.file_path))),
            "-w" => Ok(Self::Word(WordCounter::new(&config.file_path))),
            "-m" => Ok(Self::Char(CharCounter::new(&config.file_path))),
            "-a" => Ok(Self::All(AllCounter::new(&config.file_path))),
            _ => Err("Invalid counter type".into()),
        }
    }
}

impl<'a> Countable for Counter<'a> {
    fn len(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Self::Byte(byte_counter) => byte_counter.len(),
            Self::Line(line_counter) => line_counter.len(),
            Self::Word(word_counter) => word_counter.len(),
            Self::Char(char_counter) => char_counter.len(),
            Self::All(all_counter) => all_counter.len(),
        }
    }
}
