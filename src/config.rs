pub struct Config {
    pub counter_type: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the first argument

        // can be optional
        let counter_type = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a counter type"),
        };

        // can be optional
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config {
            counter_type,
            file_path,
        })
    }
}
