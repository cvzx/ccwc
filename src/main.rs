use ccwc::config::Config;
use ccwc::Wc;
use clap::{command, Arg, ArgAction, ArgMatches};

fn main() {
    let matches = parse_arguments();
    let config = Config::build_from_matches(matches);
    let result = Wc::new(config).count();

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
        .arg(Arg::new("file_path").action(ArgAction::Set))
        .get_matches()
}
