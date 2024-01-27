use crate::Source;
use std::rc::Rc;

pub enum Counter {
    Byte(Rc<Source>),
    Char(Rc<Source>),
    Line(Rc<Source>),
    Word(Rc<Source>),
}

impl Counter {
    pub fn new(counter_type: &str, source: Rc<Source>) -> Self {
        match counter_type {
            "Lines" => Self::Line(source),
            "Words" => Self::Word(source),
            "Chars" => Self::Char(source),
            "Bytes" => Self::Byte(source),
            _ => panic!("Unknown counter type"),
        }
    }

    pub fn len(&self) -> Result<usize, std::io::Error> {
        match self {
            Counter::Byte(source) => self.count_bytes(source),
            Counter::Char(source) => self.count_chars(source),
            Counter::Line(source) => self.count_lines(source),
            Counter::Word(source) => self.count_words(source),
        }
    }

    fn count_bytes(&self, source: &Source) -> Result<usize, std::io::Error> {
        Ok(source.read()?.len())
    }

    fn count_chars(&self, source: &Source) -> Result<usize, std::io::Error> {
        Ok(source.read()?.chars().count())
    }

    fn count_lines(&self, source: &Source) -> Result<usize, std::io::Error> {
        Ok(source.read()?.lines().count())
    }

    fn count_words(&self, source: &Source) -> Result<usize, std::io::Error> {
        Ok(source.read()?.split_whitespace().count())
    }
}

// Add tests for each len() type
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_bytes() {
        let source = Source::File("fixtures/lorem.txt".to_string());
        let counter = Counter::Byte(Rc::new(source));

        assert_eq!(counter.len().unwrap(), 42);
    }

    #[test]
    fn it_counts_chars() {
        let source = Source::File("fixtures/lorem.txt".to_string());
        let counter = Counter::Char(Rc::new(source));

        assert_eq!(counter.len().unwrap(), 42);
    }

    #[test]
    fn it_counts_lines() {
        let source = Source::File("fixtures/lorem.txt".to_string());
        let counter = Counter::Line(Rc::new(source));

        assert_eq!(counter.len().unwrap(), 3);
    }

    #[test]
    fn it_counts_words() {
        let source = Source::File("fixtures/lorem.txt".to_string());
        let counter = Counter::Word(Rc::new(source));

        assert_eq!(counter.len().unwrap(), 6);
    }
}
