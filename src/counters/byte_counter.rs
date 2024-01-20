use crate::Countable;
use std::error::Error;
use std::fs;

pub struct ByteCounter<'a> {
    file_path: &'a str,
}

impl<'a> ByteCounter<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self { file_path }
    }
}

impl<'a> Countable for ByteCounter<'a> {
    fn len(&self) -> Result<usize, Box<dyn Error>> {
        let file_path = &self.file_path;
        let contents = fs::read_to_string(file_path)?;

        Ok(contents.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_bytes() {
        let file_path = "fixtures/lorem.txt";
        let byte_counter = ByteCounter::new(file_path);

        assert_eq!(byte_counter.len().unwrap(), 42);
    }
}
