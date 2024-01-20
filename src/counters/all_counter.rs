// use crate::counter::Countable;
// use crate::counter::Counter;
// use crate::counters::{
// byte_counter::ByteCounter, char_counter::CharCounter, line_counter::LineCounter,
// word_counter::WordCounter,
// };
// use std::error::Error;

// pub struct AllCounter<'a> {
// counters: Vec<Counter<'a>>,
// }

// impl<'a> AllCounter<'a> {
// pub fn new(file_path: &'a str) -> Self {
// Self {
// counters: vec![
// Counter::Byte(ByteCounter::new(file_path)),
// Counter::Line(LineCounter::new(file_path)),
// Counter::Word(WordCounter::new(file_path)),
// Counter::Char(CharCounter::new(file_path)),
// ],
// }
// }
// }

// impl<'a> Countable for AllCounter<'a> {
// fn len(&self) -> Result<String, Box<dyn Error>> {
// let result = self
// .counters
// .iter()
// .map(|counter| counter.len().expect("Error counting"))
// .collect::<Vec<String>>()
// .join("  ");

// Ok(result)
// }
// }

// #[cfg(test)]
// mod tests {
// use super::*;

// #[test]
// fn it_counts_all_types() {
// let file_path = "fixtures/lorem.txt";
// let word_counter = AllCounter::new(file_path);

// assert_eq!(word_counter.len().unwrap(), "42  3  6  42");
// }
// }
