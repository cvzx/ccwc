use ccwc::config::Config;
use ccwc::Wc;
use clap::{command, Arg, ArgAction, ArgMatches};

fn main() {
    let matches = parse_arguments();

    let config = Config::build(matches).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let result = Wc::new(config).run();

    println!("{result}");
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
        .get_matches()
}
