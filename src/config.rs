use std::{env, process};

pub struct Config {
    pub query: String,
    pub path: String,
    pub content: Option<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: Vec<String>,
        content: Option<String>,
    ) -> Result<Config, &'static str> {
        let params = extract_params(&mut args);
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for p in params {
            match &*p {
                "--help" | "-h" => {
                    println!("Usage: rust-grep [OPTION] [QUERY] [FILE]");
                    process::exit(1);
                }
                "--ignore_case" | "-ic" => {
                    ignore_case = true;
                }
                _ => {
                    println!("Invalid argument at: {}", p);
                    process::exit(1);
                }
            }
        }
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
            ignore_case,
        })
    }
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
}
