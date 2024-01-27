use ccwc::config::Config;
use ccwc::Wc;
use clap::{command, Arg, ArgAction, ArgMatches};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = parse_arguments();
    let config = Config::build_from_matches(matches);

    let input = build_input(&config);
    let result = Wc::new(input, config).count();

    println!("{result}");
}

fn build_input(config: &Config) -> Box<dyn Read> {
    match &config.file_path {
        Some(file_path) => {
            Box::new(File::open(file_path).expect("Can't open file")) as Box<dyn Read>
        }
        None => Box::new(std::io::stdin()) as Box<dyn Read>,
    }
}

fn parse_arguments() -> ArgMatches {
    command!()
        .arg(
            Arg::new("count_bytes")
                .short('c')
                .long("bytes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("count_lines")
                .short('l')
                .long("lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("count_words")
                .short('w')
                .long("words")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("count_chars")
                .short('m')
                .long("chars")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("file_path").action(ArgAction::Set))
        .get_matches()
}
