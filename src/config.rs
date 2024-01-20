use clap::ArgMatches;

pub struct Config {
    pub count_bytes: bool,
    pub count_lines: bool,
    pub count_words: bool,
    pub count_chars: bool,
    pub count_all: bool,
    pub file_path: String,
}

impl Config {
    pub fn build(matches: ArgMatches) -> Result<Config, &'static str> {
        Ok(Config {
            count_bytes: matches.get_flag("count_bytes"),
            count_lines: matches.get_flag("count_lines"),
            count_words: matches.get_flag("count_words"),
            count_chars: matches.get_flag("count_chars"),
            count_all: matches.get_flag("count_all"),
            file_path: matches
                .get_one::<String>("file_path")
                .expect("missing file path")
                .to_string(),
        })
    }
}
