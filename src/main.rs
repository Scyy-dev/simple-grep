use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Args::from(&args).unwrap_or_else(|err| {
        println!("A problem occurred parsing arguments: {}", err);
        process::exit(1);
    });

    let lines = fs::read_to_string(&args.file_path).expect(format!("Could not read file {}", &args.file_path).as_str());

    println!("Searching file {} using {}", &args.query, &args.file_path);
    println!("File lines are:\n{}", lines);

    run(args);
}

#[derive(Debug, PartialEq, Clone)]
struct Args {
    query: String,
    file_path: String
}

impl Args {
    fn from(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Arg length must be 3");
        }
        Ok(Args { query: args[1].clone(), file_path: args[2].clone() })
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.file_path)?;
    println!("file contents are:\n{}", contents);
    Ok(())
}
