use std::rc::Rc;

pub enum Counter {
    Byte(Rc<String>),
    Char(Rc<String>),
    Line(Rc<String>),
    Word(Rc<String>),
}

impl Counter {
    pub fn new(counter_type: &str, content: Rc<String>) -> Self {
        match counter_type {
            "Lines" => Self::Line(content),
            "Words" => Self::Word(content),
            "Chars" => Self::Char(content),
            "Bytes" => Self::Byte(content),
            _ => panic!("Unknown counter type"),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Counter::Byte(content) => self.count_bytes(content),
            Counter::Char(content) => self.count_chars(content),
            Counter::Line(content) => self.count_lines(content),
            Counter::Word(content) => self.count_words(content),
        }
    }

    fn count_bytes(&self, content: &Rc<String>) -> usize {
        content.len()
    }

    fn count_chars(&self, content: &Rc<String>) -> usize {
        content.chars().count()
    }

    fn count_lines(&self, content: &Rc<String>) -> usize {
        content.lines().count()
    }

    fn count_words(&self, content: &Rc<String>) -> usize {
        content.split_whitespace().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_counts_bytes() {
        let content = Rc::new(fs::read_to_string("fixtures/lorem.txt").unwrap());
        let counter = Counter::Byte(content);

        assert_eq!(counter.len(), 42);
    }

    #[test]
    fn it_counts_chars() {
        let content = Rc::new(fs::read_to_string("fixtures/lorem.txt").unwrap());
        let counter = Counter::Char(content);

        assert_eq!(counter.len(), 42);
    }

    #[test]
    fn it_counts_lines() {
        let content = Rc::new(fs::read_to_string("fixtures/lorem.txt").unwrap());
        let counter = Counter::Line(content);

        assert_eq!(counter.len(), 3);
    }

    #[test]
    fn it_counts_words() {
        let content = Rc::new(fs::read_to_string("fixtures/lorem.txt").unwrap());
        let counter = Counter::Word(content);

        assert_eq!(counter.len(), 6);
    }
}
