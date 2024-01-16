use ccwc::config::Config;

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = ccwc::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
