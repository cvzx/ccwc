use ccwc::config::Config;
use clap::{command, Arg, ArgAction};

fn main() {
    let matches = parse_arguments();

    // let counter_type = matches
    // .get_one::<String>("counter_type")
    // .expect("Missing counter argument");

    // let file_path = matches
    // .get_one::<String>("file_path")
    // .expect("Missing file path");

    let config = Config::build(matches).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    // if let Err(e) = ccwc::run(config) {
    // eprintln!("Application error: {}", e);
    // std::process::exit(1);
    // }
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
        .arg(
            Arg::new("count_all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("file_path"))
        .get_matches();
}
