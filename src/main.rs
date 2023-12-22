mod ast;
mod constants;
mod interpreter;
mod literal_type;
mod run;
mod run_file;
mod run_prompt;
mod scanner;
mod token;
mod token_type;
mod util;

use run::run;
use run_file::run_file;
use run_prompt::run_prompt;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: wrig [script]");

            // Invalid command line argument exit code
            exit(64);
        }
    };

    run(&input).expect("Unable to run the program!");
}
