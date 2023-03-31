use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (regex, filename) = parse_args(&args);

    // error handling handled later
    // if args.len() < 3 { panic!("Regex and filename not specified") }

    let lines = fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    println!("Searching file {} using {}", filename, regex);
    println!("File lines are:\n{}", lines);
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let regex = &args[1];
    let filename = &args[2];

    (&regex, &filename)
}
