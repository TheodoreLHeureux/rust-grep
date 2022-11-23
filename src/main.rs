use rust_grep::Config;
use std::env;
use std::process;

fn main() {
    let args  = env::args();
    let stdin = rust_grep::get_piped();
    let config: Config;

    if stdin.is_empty() {
        config = Config::build(args, None).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    } else {
        config = Config::build(args, Some(stdin)).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    }

    if let Err(e) = rust_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
