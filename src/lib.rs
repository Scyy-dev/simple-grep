use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
pub struct Args {
    query: String,
    file_path: String
}

impl Args {
    pub fn from(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Arg length must be 3");
        }
        Ok(Args { query: args[1].clone(), file_path: args[2].clone() })
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.file_path)?;
    println!("file contents are:\n{}", contents);
    Ok(())
}