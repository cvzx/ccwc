use crate::Countable;
use crate::Source;
use std::io;
use std::rc::Rc;

pub struct CharCounter {
    source: Rc<dyn Source>,
}

impl CharCounter {
    pub fn new(source: Rc<dyn Source>) -> Self {
        Self { source }
    }
}

impl Countable for CharCounter {
    fn len(&self) -> Result<usize, io::Error> {
        let contents = self.source.read()?;

        Ok(contents.chars().count())
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
    fn it_counts_chars() {
        let source = MockedSource {};
        let char_counter = CharCounter::new(Rc::new(source));

        assert_eq!(char_counter.len().unwrap(), 42);
    }
}
