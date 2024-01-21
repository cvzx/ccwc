use crate::Countable;
use std::error::Error;
use std::fs;
use std::rc::Rc;

pub struct WordCounter {
    file_path: Rc<String>,
}

impl WordCounter {
    pub fn new(file_path: Rc<String>) -> Self {
        Self { file_path }
    }
}

impl Countable for WordCounter {
    fn len(&self) -> Result<usize, Box<dyn Error>> {
        let file_path: &str = &self.file_path;
        let contents = fs::read_to_string(file_path)?;

        Ok(contents.split_whitespace().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_words() {
        let file_path = Rc::new("fixtures/lorem.txt".to_string());
        let word_counter = WordCounter::new(file_path);

        assert_eq!(word_counter.len().unwrap(), 6);
    }
}
