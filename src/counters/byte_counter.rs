use crate::Countable;
use crate::Source;
use std::io;
use std::rc::Rc;

pub struct ByteCounter {
    source: Rc<dyn Source>,
}

impl ByteCounter {
    pub fn new(source: Rc<dyn Source>) -> Self {
        Self { source }
    }
}

impl Countable for ByteCounter {
    fn len(&self) -> Result<usize, io::Error> {
        let contents = self.source.read()?;

        Ok(contents.len())
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
    fn it_counts_bytes() {
        let source = MockedSource {};
        let byte_counter = ByteCounter::new(Rc::new(source));

        assert_eq!(byte_counter.len().unwrap(), 42);
    }
}
