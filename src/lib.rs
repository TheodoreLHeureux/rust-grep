use std::{env, process};
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};

pub struct Config {
    pub query: String,
    pub path: String,
    pub content: Option<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: Vec<String>, content: Option<String>) -> Result<Config, &'static str> {
        let params = extract_params(&mut args);

        if params.contains(&"--help".to_string()) || params.contains(&"-h".to_string()) {

            println!("Usage: rust-grep [OPTION] [QUERY] [FILE]");
        
            process::exit(1);
        }

        if (args.len() < 3 && content == None) || args.len() < 2 {
            return Err("Not enough arguments.");
        }

        let mut path = String::new();

        if args.len() > 2 {
            path = args[2].clone();
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: args[1].clone(),
            path,
            content,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String;

    match config.content {
        None => content = fs::read_to_string(config.path)?,
        Some(v) => content = v,
    }

    let result: Vec<&str>;

    if config.ignore_case {
        result = search_case_insensitive(&config.query, &content)
    } else {
        result = search(&config.query, &content);
    }

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn extract_params(args: &mut Vec<String>) -> Vec<String> {
    let mut params = Vec::new();
    let mut indices: Vec<usize> = Vec::new();

    for i in 0..args.len() {
        if args[i].starts_with('-') {
            indices.push(i);
            params.push(args[i].clone());
        }
    }

    for i in indices {
        args.remove(i);
    }

    params
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

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
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

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
