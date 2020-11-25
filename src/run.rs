use crate::{parser::Parser, scanner::Scanner};
use std::fs;
use std::io;
use std::io::Write;

pub fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let expression = parser.parse();

    if let Some(e) = expression {
        println!("{:?}", e)
    }
}

pub fn run_file(path: &str) {
    let filecontents = fs::read_to_string(path).expect("Error reading file");
    run(filecontents);
}

pub fn run_prompt() {
    loop {
        let mut input = String::new();
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        run(input);
    }
}
