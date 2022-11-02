use std::error::Error;
use std::fs;
use std::io::{self, BufRead};

pub struct Config {
    pub query: String,
    pub path: String,
    content: Option<String>,
}

impl Config {
    pub fn build(args: Vec<String>, content: Option<String>) -> Result<Config, &'static str> {
        if (args.len() < 3 && content == None) || args.len() < 2 {
            return Err("Not enough arguments.");
        }

        let mut path = String::new();

        if args.len() > 2 {
            path = args[2].clone();
        }

        Ok(Config {
            query: args[1].clone(),
            path,
            content,
        })
    }
}

pub fn run(mut config: Config) -> Result<(), Box<dyn Error>> {
    if let None = config.content {
        config.content = Some(fs::read_to_string(config.path)?);
    }

    for line in search(&config.query, &config.content.unwrap()) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn get_piped() -> String {
    let mut input = String::new();

    loop {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        if atty::is(atty::Stream::Stdin) {
            return input;
        }

        if let Ok(n) = handle.read_line(&mut input) {
            if n == 0 {
                return input;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build() {
        let args = vec![
            String::from("target/debug/rust_grep"),
            String::from("query"),
            String::from("path"),
        ];

        let config = Config::build(args.clone(), None).unwrap();

        assert_eq!(config.query, args[1]);
        assert_eq!(config.path, args[2]);

        let args = vec![
            String::from("target/debug/rust_grep"),
            String::from("43244dgdsd"),
            String::from("243rewfdd"),
        ];

        let config = Config::build(args.clone(), None).unwrap();

        assert_eq!(config.query, args[1]);
        assert_eq!(config.path, args[2]);
    }

    #[test]
    fn config_build_not_enough_args() {
        let args = vec![String::from("target/debug/rust_grep")];

        let config = Config::build(args, None);

        assert!(config.is_err());

        let args = vec![
            String::from("target/debug/rust_grep"),
            String::from("query"),
        ];

        let config = Config::build(args, None);

        assert!(config.is_err());
    }

    #[test]
    fn one_result() {
        let query = String::from("duct");
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }
}
