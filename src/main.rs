use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let path = &args[2];

    let content = fs::read_to_string(path).expect("Error while reading file.");

    println!("Contents : \n{}", content);
}
