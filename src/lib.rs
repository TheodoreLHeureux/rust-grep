use atty::Stream;
use std::error::Error;
use std::io::{stdin, BufRead, Stdin};
use std::{fs, io};

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn build(args: Box<[String]>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    for line in search(&config.query, &content) {
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

pub fn get_piped() -> io::Result<String> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock(); // locking is optional

    let mut line = String::new();

    // Could also `match` on the `Result` if you wanted to handle `Err`
    while let Ok(n_bytes) = stdin.read_line(&mut line) {
        if n_bytes == 0 {
            break;
        }
        println!("{}", line);
        line.clear();
    }
    Ok(line)
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

        let config = Config::build(Box::from(args.clone())).unwrap();

        assert_eq!(config.query, args[1]);
        assert_eq!(config.path, args[2]);

        let args = vec![
            String::from("target/debug/rust_grep"),
            String::from("43244dgdsd"),
            String::from("243rewfdd"),
        ];

        let config = Config::build(Box::from(args.clone())).unwrap();

        assert_eq!(config.query, args[1]);
        assert_eq!(config.path, args[2]);
    }

    #[test]
    fn config_build_not_enough_args() {
        let args = vec![String::from("target/debug/rust_grep")];

        let config = Config::build(Box::from(args));

        assert!(config.is_err());

        let args = vec![
            String::from("target/debug/rust_grep"),
            String::from("query"),
        ];

        let config = Config::build(Box::from(args));

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
