use crate::Countable;
use crate::Source;
use std::io;
use std::rc::Rc;

pub struct LineCounter {
    source: Rc<dyn Source>,
}

impl LineCounter {
    pub fn new(source: Rc<dyn Source>) -> Self {
        Self { source }
    }
}

impl Countable for LineCounter {
    fn len(&self) -> Result<usize, io::Error> {
        let contents = self.source.read()?;

        Ok(contents.lines().count())
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
    fn it_counts_lines() {
        let source = MockedSource {};
        let line_counter = LineCounter::new(Rc::new(source));

        assert_eq!(line_counter.len().unwrap(), 3);
    }
}
