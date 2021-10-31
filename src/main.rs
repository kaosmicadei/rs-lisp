fn main() {
    let args: Vec<String> = std::env::args().collect();
        if args.len() > 2 {
            eprint!("Usage: rs-lisp <file>");
            std::process::exit(64);
        } else if args.len() == 2 {
            parse_code(&args[1]);
        } else {
            repl();
        }
}

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
#[derive(Parser)]
#[grammar = "lisp.pest" ]
struct LispParser;

#[derive(Debug)]
enum Sexpr {
    Bool(bool),
    Number(f64),
    Text(String),
    Symbol(String),
    List(Vec<Sexpr>),
}

use pest::iterators::Pair;

fn parse_sexpr(pair: Pair<Rule>) -> Sexpr {
    match pair.as_rule() {
        Rule::bool => Sexpr::Bool(pair.as_str().parse().unwrap()),
        Rule::number => Sexpr::Number(pair.as_str().parse().unwrap()),
        Rule::text => Sexpr::Text(pair.as_str().to_string()),
        Rule::symbol => Sexpr::Symbol(pair.as_str().to_string()),
        Rule::list => Sexpr::List(pair.into_inner().map(parse_sexpr).collect()),
        _ => unreachable!(),
    }
}

fn parse_code(file: &str) -> Vec<Sexpr> {
    use std::fs;
    let input = fs::read_to_string(file).expect("failed to read the file");
    let code = LispParser::parse(Rule::program, &input).expect("fail to parse the code").next().unwrap();

    code.into_inner()
        .map(parse_sexpr)
        .collect()
}

pub fn repl() {
    use std::io::{self,Write};

    let mut line = String::new();
    loop {
        print!("λ. ");
        io::stdout().flush().expect("Failed print output");
        
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(n) if n == 0 => break,
            Ok(_) =>  {
                let expr = LispParser::parse(Rule::sexpr, &line);
                println!("=> {:?}", expr);
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}


