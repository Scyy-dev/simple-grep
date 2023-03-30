use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // error handling handled later
    // if args.len() < 3 { panic!("Regex and filename not specified") }

    let regex = &args[1];
    let file = &args[2];

    let lines = fs::read_to_string(file).expect(format!("Could not read file {}", file).as_str());

    println!("Searching file {} using {}", file, regex);
    println!("File lines are:\n{}", lines);
}
