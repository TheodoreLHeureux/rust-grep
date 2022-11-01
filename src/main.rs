use rust_grep::Config;
use std::env;
use std::io;
use std::process;

fn main() {
    println!("{}", rust_grep::get_piped().unwrap());

    // let args: Vec<String> = env::args().collect();
    // let stdin = rust_grep::get_stdin(io::stdin());
    // let mut config: Option<Config> = None;

    // if let Ok(content) = stdin {
    //     config = Some(
    //         Config::build(Box::from([String::new(), content, args[1].clone()])).unwrap_or_else(
    //             |err| {
    //                 println!("Problem parsing arguments: {err}");
    //                 process::exit(1);
    //             },
    //         ),
    //     );
    // }
    // if let None = config {
    //     config = Some(Config::build(Box::from(args)).unwrap_or_else(|err| {
    //         println!("Problem parsing arguments: {err}");
    //         process::exit(1);
    //     }));
    // }

    // let config = config.unwrap();

    // if let Err(e) = rust_grep::run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }
}
