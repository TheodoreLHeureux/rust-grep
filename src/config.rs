pub use parameters::Parameters;

pub mod parameters;

pub struct Config {
    pub query: String,
    pub path: String,
    pub content: Option<String>,
    pub params: Parameters,
}

impl Config {
    pub fn build(
        mut args_iter: impl Iterator<Item = String>,
        content: Option<String>,
    ) -> Result<Config, String> {
        let mut args: Vec<String> = Vec::new();
        let params = Parameters::build(&mut args_iter, &mut args)?;

        if (args.len() < 3 && content == None) || args.len() < 2 {
            return Err("Not enough arguments.".to_string());
        }

        let mut path = String::new();

        if args.len() > 2 {
            path = args[2].clone();
        }

        Ok(Config {
            query: args[1].clone(),
            path,
            content,
            params,
        })
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
            String::from("-ic"),
        ];

        let config = Config::build(args.clone().into_iter(), None).unwrap();

        assert_eq!(config.query, args[1]);
        assert_eq!(config.path, args[2]);

        let args = vec![
            String::from("target/debug/rust_grep"),
            String::from("43244dgdsd"),
            String::from("243rewfdd"),
        ];

        let config = Config::build(args.clone().into_iter(), None).unwrap();

        assert_eq!(config.query, args[1]);
        assert_eq!(config.path, args[2]);
    }

    #[test]
    fn config_build_not_enough_args() {
        let args = vec![String::from("target/debug/rust_grep")];

        let config = Config::build(args.into_iter(), None);

        assert!(config.is_err());

        let args = vec![
            String::from("target/debug/rust_grep"),
            String::from("query"),
        ];

        let config = Config::build(args.into_iter(), None);

        assert!(config.is_err());
    }
}
