use clap::ArgMatches;

pub struct Config<'a> {
    pub flags: Vec<&'a str>,
    pub file_path: String,
}

impl<'a> Config<'a> {
    pub fn build(matches: ArgMatches) -> Config<'a> {
        let flags = vec!["count_lines", "count_words", "count_chars", "count_bytes"]
            .into_iter()
            .filter(|&flag| matches.get_flag(flag))
            .collect();

        let file_path = matches
            .get_one::<String>("file_path")
            .expect("missing file path")
            .to_string();

        Config { flags, file_path }
    }
}
