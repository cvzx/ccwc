use std::fs;
use std::io::Read;

pub enum Source {
    File(String),
    StdIn,
}

impl Source {
    pub fn read(&self) -> Result<String, std::io::Error> {
        match self {
            Source::File(file_path) => self.read_file(file_path),
            Source::StdIn => self.read_stdin(),
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Source::File(file_path) => file_path,
            Source::StdIn => "",
        }
    }

    fn read_file(&self, file_path: &String) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }

    fn read_stdin(&self) -> Result<String, std::io::Error> {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}
