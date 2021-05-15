mod scanner;

use std::env::args;
use std::process::exit;
use std::fs::read_to_string;
use std::io::{ stdin, stdout, Write };
use std::sync::atomic::{AtomicBool, Ordering};

use scanner::{ Scanner, token::Token };

/* Avoids passing in self as argument which allows use to 
continue with the tutorial without changing the structure
of the code too much for error and report function. */
static HAD_ERROR: AtomicBool = AtomicBool::new(false);

struct Lox {
}

impl Lox {
    fn main(&mut self) {
        let args: Vec<String> = args().collect();
    
        if args.len() > 2 {
            println!("Usage: jlox [script]");
            exit(64);
        } else if args.len() == 2 {
            self.run_file(args[1].clone());
        } else {
            self.run_prompt();
        }
    }
    
    fn run_file(&self, path: String) {
        match read_to_string(path) {
            Ok(source) => self.run(source),
            Err(error) => panic!("{}", error)
        };
    
        if HAD_ERROR.load(Ordering::Relaxed) {
            exit(64);
        }
    }
    
    fn run_prompt(&mut self) {
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
                Ok(_) => self.run(input),
                Err(error) => {
                    panic!("{}", error);
                }
            }
    
            HAD_ERROR.store(false, Ordering::Relaxed);
        }
    }
    
    fn run(&self, source: String) {
        let mut scanner: Scanner = Scanner::new(source);
        let tokens: Vec::<Token> = scanner.scan_tokens();
    
        for token in tokens {
            println!("{:?}", token);
        }
    }
    
    fn error(line: u32, message: String) {
        Lox::report(line, String::new(), message);
    }
    
    fn report (line: u32, location: String, message: String) {
        println!("[line {}] Error {}: {}", line, location, message);
        HAD_ERROR.store(true, Ordering::Relaxed);
    }
}

fn main() {
    let mut lox = Lox { };
    lox.main();
}