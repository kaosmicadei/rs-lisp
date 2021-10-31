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

fn main() {
    let ast = parse_code("example.lisp");
    println!("{:?}", ast);
}
