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

    if let Err(e) = run(args) {
        println!("An error occurred while running the application: {e}");
        process::exit(1);
    }

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
