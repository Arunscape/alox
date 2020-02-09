use std::env;
use std::process::exit;
use std::usize::MAX;

mod error;
mod run;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run::run_file(&args[1]),
        (2..=MAX) => {
            println!("Usage: alox [script]");
            exit(64);
        }
        _ => run::run_prompt(),
    }
}
