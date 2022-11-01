use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a String,
    pub path: &'a String,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = &args[1];
        let path = &args[2];

        Ok(Config { query, path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    println!("Contents of {}: \n{}", config.query, content);

    Ok(())
}
