#![feature(half_open_range_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(precise_pointer_size_matching)]
#![allow(dead_code)]
use std::env;
use std::process::exit;

mod error;
mod run;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => run::run_prompt(),
        1 => run::run_file(&args[0]),
        (2..) => {
            println!("Usage: alox [script]");
            exit(64);
        }
    }
}
