mod scanner;

use std::env::args;
use std::process::exit;
use std::fs::read_to_string;
use std::io::{ stdin, stdout, Write };

use scanner::{ Scanner, token::Token };

fn main() {
    let args: Vec<String> = args().collect();
    let mut had_error: bool = false;

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone(), had_error);
    } else {
        run_prompt(&mut had_error);
    }
}

fn run_file(path: String, had_error: bool) {
    match read_to_string(path) {
        Ok(source) => run(source),
        Err(error) => panic!("{}", error)
    };

    if had_error {
        exit(64);
    }
}

fn run_prompt(had_error: &mut bool) {
    loop {
        print!("> ");
        /* Stdout is line buffered. Flush is triggered on a new line.
        We need to flush out the currently line manually.*/ 
        match stdout().flush() {
            Ok(_) => { },
            Err(error) => {
                panic!("{}", error);
            }
        };
        
        let mut input = String::new();
        
        match stdin().read_line(&mut input) {
            Ok(_) => run(input),
            Err(error) => {
                panic!("{}", error);
            }
        }

        *had_error = false;
    }
}

fn run(source: String) {
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec::<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line: u32, message: String, had_error: &mut bool) {
    report(line, String::from(""), message, had_error);
}

fn report (line: u32, location: String, message: String, had_error: &mut bool) {
    println!("[line {}] Error {}: {}", line, location, message);
    *had_error = true;
}