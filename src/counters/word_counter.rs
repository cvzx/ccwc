use crate::counter::Countable;
use std::error::Error;
use std::fs;

pub struct WordCounter<'a> {
    file_path: &'a str,
}

impl<'a> WordCounter<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self { file_path }
    }
}

impl<'a> Countable for WordCounter<'a> {
    fn len(&self) -> Result<String, Box<dyn Error>> {
        let file_path = &self.file_path;
        let contents = fs::read_to_string(file_path)?;

        Ok(contents.split_whitespace().count().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_words() {
        let file_path = "fixtures/lorem.txt";
        let word_counter = WordCounter::new(file_path);

        assert_eq!(word_counter.len().unwrap(), "6");
    }
}
