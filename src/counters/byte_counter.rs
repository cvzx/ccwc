use crate::Countable;
use std::error::Error;
use std::fs;
use std::rc::Rc;

pub struct ByteCounter {
    file_path: Rc<String>,
}

impl ByteCounter {
    pub fn new(file_path: Rc<String>) -> Self {
        Self { file_path }
    }
}

impl Countable for ByteCounter {
    fn len(&self) -> Result<usize, Box<dyn Error>> {
        let file_path: &str = &self.file_path;
        let contents = fs::read_to_string(file_path)?;

        Ok(contents.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_bytes() {
        let file_path = Rc::new("fixtures/lorem.txt".to_string());
        let byte_counter = ByteCounter::new(file_path);

        assert_eq!(byte_counter.len().unwrap(), 42);
    }
}
