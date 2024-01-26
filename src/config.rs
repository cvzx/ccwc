use clap::ArgMatches;

pub struct Config {
    pub count_lines: bool,
    pub count_words: bool,
    pub count_chars: bool,
    pub count_bytes: bool,
    pub file_path: Option<String>,
}

impl Config {
    pub fn build_from_matches(matches: ArgMatches) -> Self {
        Config {
            count_lines: matches.get_flag("count_lines"),
            count_words: matches.get_flag("count_words"),
            count_chars: matches.get_flag("count_chars"),
            count_bytes: matches.get_flag("count_bytes"),
            file_path: matches.get_one::<String>("file_path").cloned(),
        }
    }

    pub fn flags(&self) -> Vec<(&str, bool)> {
        vec![
            ("count_lines", self.count_lines),
            ("count_words", self.count_words),
            ("count_chars", self.count_chars),
            ("count_bytes", self.count_bytes),
        ]
    }
}
