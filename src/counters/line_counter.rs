use crate::Countable;
use std::error::Error;
use std::fs;

pub struct LineCounter<'a> {
    file_path: &'a str,
}

impl<'a> LineCounter<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self { file_path }
    }
}

impl<'a> Countable for LineCounter<'a> {
    fn len(&self) -> Result<usize, Box<dyn Error>> {
        let file_path = &self.file_path;
        let contents = fs::read_to_string(file_path)?;

        Ok(contents.lines().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_lines() {
        let file_path = "fixtures/lorem.txt";
        let line_counter = LineCounter::new(file_path);

        assert_eq!(line_counter.len().unwrap(), 3);
    }
}
