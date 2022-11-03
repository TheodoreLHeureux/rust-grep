use std::{env, process};

pub struct Parameters {
    pub ignore_case: bool,
}

impl Parameters {
    pub fn build(args: &mut Vec<String>) -> Result<Parameters, String> {
        let params = extract_params(args);
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for p in params {
            match &*p {
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
