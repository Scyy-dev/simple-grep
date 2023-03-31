use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = parse_args(&args);

    // error handling handled later
    // if args.len() < 3 { panic!("Regex and filename not specified") }

    dbg!(&args);

    let lines = fs::read_to_string(&args.file_path).expect(format!("Could not read file {}", &args.file_path).as_str());

    println!("Searching file {} using {}", &args.query, &args.file_path);
    println!("File lines are:\n{}", lines);
}

#[derive(Debug, PartialEq, Clone)]
struct Args {
    query: String,
    file_path: String
}

impl Args {
    fn new(regex: &String, file_path: &String) -> Args {
        Args { query: regex.clone(), file_path: file_path.clone() }
    }
}

fn parse_args(args: &[String]) -> Args {
    Args::new(&args[1], &args[2])
}
