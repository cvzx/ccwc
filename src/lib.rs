pub mod config;
mod counter;
mod counters {
    pub mod all_counter;
    pub mod byte_counter;
    pub mod char_counter;
    pub mod line_counter;
    pub mod word_counter;
}

use crate::config::Config;
use crate::counter::{Countable, Counter};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let counter = Counter::new(&config)?;

    println!("{} {}", counter.len()?, config.file_path);

    Ok(())
}
