use std::fs;
use std::io;
use std::io::Write;

pub fn run(source: &str){
    for token in source.split_whitespace(){
        println!("{}", token);
    }
}

pub fn run_file(path: &str){
    let filecontents = fs::read_to_string(path).expect("Error reading file");
    run(&filecontents);
}

pub fn run_prompt(){
    loop {
        let mut input = String::new();
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        run(&input);
    }
}
