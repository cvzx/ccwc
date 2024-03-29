use std::sync::Arc;

pub enum Counter {
    Byte(Arc<String>),
    Char(Arc<String>),
    Line(Arc<String>),
    Word(Arc<String>),
}

impl Counter {
    pub fn new(counter_type: &str, content: Arc<String>) -> Self {
        match counter_type {
            "lines" => Self::Line(content),
            "words" => Self::Word(content),
            "chars" => Self::Char(content),
            "bytes" => Self::Byte(content),
            _ => panic!("Unknown counter type"),
        }
    }

    pub fn len(&self) -> (String, usize) {
        match self {
            Counter::Byte(content) => self.count_bytes(content),
            Counter::Char(content) => self.count_chars(content),
            Counter::Line(content) => self.count_lines(content),
            Counter::Word(content) => self.count_words(content),
        }
    }

    fn count_bytes(&self, content: &Arc<String>) -> (String, usize) {
        (String::from("bytes"), content.len())
    }

    fn count_chars(&self, content: &Arc<String>) -> (String, usize) {
        (String::from("chars"), content.chars().count())
    }

    fn count_lines(&self, content: &Arc<String>) -> (String, usize) {
        (String::from("lines"), content.lines().count())
    }

    fn count_words(&self, content: &Arc<String>) -> (String, usize) {
        (String::from("words"), content.split_whitespace().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_counts_bytes() {
        let content = Arc::new(fs::read_to_string("fixtures/lorem.txt").unwrap());
        let counter = Counter::Byte(content);

        assert_eq!(counter.len(), (String::from("bytes"), 42));
    }

    #[test]
    fn it_counts_chars() {
        let content = Arc::new(fs::read_to_string("fixtures/lorem.txt").unwrap());
        let counter = Counter::Char(content);

        assert_eq!(counter.len(), (String::from("chars"), 42));
    }

    #[test]
    fn it_counts_lines() {
        let content = Arc::new(fs::read_to_string("fixtures/lorem.txt").unwrap());
        let counter = Counter::Line(content);

        assert_eq!(counter.len(), (String::from("lines"), 3));
    }

    #[test]
    fn it_counts_words() {
        let content = Arc::new(fs::read_to_string("fixtures/lorem.txt").unwrap());
        let counter = Counter::Word(content);

        assert_eq!(counter.len(), (String::from("words"), 6));
    }
}
