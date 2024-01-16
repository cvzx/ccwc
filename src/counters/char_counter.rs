use crate::counter::Countable;
use std::error::Error;
use std::fs;

pub struct CharCounter<'a> {
    file_path: &'a str,
}

impl<'a> CharCounter<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self { file_path }
    }
}

impl<'a> Countable for CharCounter<'a> {
    fn len(&self) -> Result<String, Box<dyn Error>> {
        let file_path = &self.file_path;
        let contents = fs::read_to_string(file_path)?;

        Ok(contents.chars().count().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_chars() {
        let file_path = "fixtures/lorem.txt";
        let char_counter = CharCounter::new(file_path);

        assert_eq!(char_counter.len().unwrap(), "42");
    }
}
