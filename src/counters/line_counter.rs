use crate::Countable;
use std::error::Error;
use std::fs;
use std::rc::Rc;

pub struct LineCounter {
    file_path: Rc<String>,
}

impl LineCounter {
    pub fn new(file_path: Rc<String>) -> Self {
        Self { file_path }
    }
}

impl Countable for LineCounter {
    fn len(&self) -> Result<usize, Box<dyn Error>> {
        let file_path: &str = &self.file_path;
        let contents = fs::read_to_string(file_path)?;

        Ok(contents.lines().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_lines() {
        let file_path = Rc::new("fixtures/lorem.txt".to_string());
        let line_counter = LineCounter::new(file_path);

        assert_eq!(line_counter.len().unwrap(), 3);
    }
}
