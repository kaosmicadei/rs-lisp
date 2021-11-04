#![allow(dead_code)]
mod parser;
mod eval;

use std::io::{self,Write};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        eprint!("Usage: rs-lisp <file>");
        std::process::exit(64);
    } else if args.len() == 2 {
        let mut env = HashMap::new();
        for expr in parser::parse_code(&args[1]) {
            eval::eval(&mut env, &expr);
        }
    } else {
        println!("Press Ctrl-D to quit.");
        repl();
    }
}

pub fn repl() {
    let mut env = HashMap::new();

    let mut line = String::new();
    loop {
        print!("λ. ");
        io::stdout().flush().expect("Failed print output");
        
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(n) if n == 0 => break,
            Ok(_) =>  {
                let expr = parser::parse_line(&line);
                println!("=> {}\n", eval::eval(&mut env, &expr));
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}