use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    println!("Contents : \n{}", content);

    Ok(())
}

struct Config<'a> {
    query: &'a String,
    path: &'a String,
}

impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = &args[1];
        let path = &args[2];

        Ok(Config { query, path })
    }
}
