use std::env;
use std::process;

use simple_grep::Args;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Args::from(&args).unwrap_or_else(|err| {
        println!("A problem occurred parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = simple_grep::run(args) {
        println!("An error occurred while running the application: {e}");
        process::exit(1);
    }

}
