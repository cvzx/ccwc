use crate::Countable;
use std::error::Error;
use std::fs;
use std::rc::Rc;

pub struct CharCounter {
    file_path: Rc<String>,
}

impl CharCounter {
    pub fn new(file_path: Rc<String>) -> Self {
        Self { file_path }
    }
}

impl Countable for CharCounter {
    fn len(&self) -> Result<usize, Box<dyn Error>> {
        let file_path: &str = &self.file_path;
        let contents = fs::read_to_string(file_path)?;

        Ok(contents.chars().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_chars() {
        let file_path = Rc::new("fixtures/lorem.txt".to_string());
        let char_counter = CharCounter::new(file_path);

        assert_eq!(char_counter.len().unwrap(), 42);
    }
}
