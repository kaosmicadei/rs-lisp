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

#[derive(Debug, Clone)]
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
    let input = fs::read_to_string(file).expect("failed to read file");
    let code = LispParser::parse(Rule::program, &input).expect("fail to parse code").next().unwrap();

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
                let expr = LispParser::parse(Rule::sexpr, &line).expect("fail to parse code").next().unwrap();
                println!("=> {:?}", eval(parse_sexpr(expr)));
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

type Function = fn(&[Sexpr]) -> Sexpr;
use std::collections::HashMap;

fn eval(expr: Sexpr) -> Sexpr {
    let mut primitives: HashMap<&str, Function> = HashMap::new();
    primitives.insert("+", add);
    primitives.insert("-", subtract);
    primitives.insert("*", multiply);
    primitives.insert("/", divide);

    match expr {
        val@Sexpr::Bool(_) => val,
        val@Sexpr::Number(_) => val,
        val@Sexpr::Text(_) => val,
        Sexpr::List(a) => {
            match &a[0] {
                Sexpr::Symbol(s) => {
                    let f = primitives.get(s.as_str()).unwrap();
                    let args = &a[1..];
                    f(args)
                },
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn add(args: &[Sexpr]) -> Sexpr {
    let res = args.iter()
                  .map(|elem| {
                      match elem {
                          Sexpr::Number(x) => x,
                          _ => unimplemented!(),
                      }
                  })
                  .sum();
    Sexpr::Number(res)
}

fn subtract(args: &[Sexpr]) -> Sexpr {
    let res = args.iter()
                  .map(|elem| {
                      match elem {
                          Sexpr::Number(x) => -x,
                          _ => unimplemented!(),
                      }
                  })
                  .sum();
    Sexpr::Number(res)
}

fn multiply(args: &[Sexpr]) -> Sexpr {
    let res = args.iter()
                  .map(|elem| {
                      match elem {
                          Sexpr::Number(x) => x,
                          _ => unimplemented!(),
                      }
                  })
                  .product();
    Sexpr::Number(res)
}

fn divide(args: &[Sexpr]) -> Sexpr {
    let res = args.iter()
                  .map(|elem| {
                      match elem {
                          Sexpr::Number(x) => 1.0/x,
                          _ => unimplemented!(),
                      }
                  })
                  .product();
    Sexpr::Number(res)
}