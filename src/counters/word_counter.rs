use crate::Countable;
use crate::Source;
use std::io;
use std::rc::Rc;

pub struct WordCounter {
    source: Rc<dyn Source>,
}

impl WordCounter {
    pub fn new(source: Rc<dyn Source>) -> Self {
        Self { source }
    }
}

impl Countable for WordCounter {
    fn len(&self) -> Result<usize, io::Error> {
        let contents = self.source.read()?;

        Ok(contents.split_whitespace().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    struct MockedSource {}

    impl Source for MockedSource {
        fn read(&self) -> Result<String, io::Error> {
            fs::read_to_string("fixtures/lorem.txt")
        }
    }

    #[test]
    fn it_counts_words() {
        let source = MockedSource {};
        let word_counter = WordCounter::new(Rc::new(source));

        assert_eq!(word_counter.len().unwrap(), 6);
    }
}
