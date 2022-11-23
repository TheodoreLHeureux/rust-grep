use std::{env, process};

pub struct Parameters {
    pub ignore_case: bool,
}

impl Parameters {
    pub fn build(
        args_iterator: &mut impl Iterator<Item = String>,
        args: &mut Vec<String>,
    ) -> Result<Parameters, String> {
        let mut params: Vec<String> = Vec::new();
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        while let Some(arg) = args_iterator.next() {
            if arg.starts_with("-") {
                params.push(arg);
            } else {
                args.push(arg);
            }
        }

        for p in params {
            match &*p {
                "--version" | "-v" => {
                    println!("{} ({})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),);
                    process::exit(0);
                }
                "--help" | "-h" => {
                    println!("Usage: rust-grep [OPTION] [QUERY] [FILE]");
                    process::exit(0);
                }
                "--ignore_case" | "-ic" => {
                    ignore_case = true;
                }
                _ => {
                    return Err(format!("Invalid parameter ({p})"));
                }
            }
        }

        Ok(Parameters { ignore_case })
    }
}
