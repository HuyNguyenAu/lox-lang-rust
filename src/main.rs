mod scanner;

use std::env::args;
use std::process::exit;
use std::fs::read_to_string;
use std::io::{ stdin, stdout, Write };

use scanner::{ Scanner, token::Token };


fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        run_prompt();
    }
}

fn run_file(path: String) {
    match read_to_string(path) {
        Ok(source) => run(source),
        Err(error) => panic!("{}", error)
    };
}

fn run_prompt() {
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
    }
}

fn run(source: String) {
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec::<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}