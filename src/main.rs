use std::fs;
use std::env;
use std::process;
use std::io::prelude::*;

mod token;
mod scanner;

fn report(line: i8, loc: String, message: String) -> () {

}

fn error(line: i8, message: String) -> () {

}

fn run(source: String, had_error: &bool) -> () {
    let tokens = scanner::scan_tokens(source);
}

fn run_file(file_str: &String, had_error: &bool) -> () {
    let file_contents : String = fs::read_to_string(file_str).unwrap(); 
    run(file_contents, had_error);
    if *had_error {
        process::exit(65);
    }
} 

fn run_prompt(had_error: &mut bool) -> () {
    print!("> ");
    std::io::stdout().flush().expect("failed to flush buffer");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let unwr : String = line.unwrap();
        println!("{}", unwr);
        print!("> ");
        std::io::stdout().flush().expect("failed to flush buffer");
        run(unwr, had_error);
        *had_error = false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut had_error : bool = false;

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1], &had_error);
    } else {
        run_prompt(&mut had_error);
    }
}
