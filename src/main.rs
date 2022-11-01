use rust_grep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {} in {}:\n", config.query, config.path);

    if let Err(e) = rust_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
