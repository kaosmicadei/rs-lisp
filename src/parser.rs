use pest;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lisp.pest" ]
pub struct LispParser;

#[derive(Debug, Clone)]
pub enum Sexpr {
    List(Vec<Sexpr>),
    // Atomic values
    Bool(bool),
    Number(f64),
    Text(String),
    Symbol(String),
}

use std::fmt;
impl fmt::Display for Sexpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::Text(t) => write!(f, "{}", t),
            Self::Symbol(s) => write!(f, "{}", s),
            Self::List(l) => {
                let elems: Vec<String> = l.iter().map(ToString::to_string).collect();
                write!(f, "({})", elems.join(" "))
            },
        }
    }
}

use pest::iterators::Pair;

pub fn parse_sexpr(pair: Pair<Rule>) -> Sexpr {
    match pair.as_rule() {
        Rule::bool => Sexpr::Bool(pair.as_str().parse().unwrap()),
        Rule::number => Sexpr::Number(pair.as_str().parse().unwrap()),
        Rule::text => Sexpr::Text(pair.as_str().to_string()),
        Rule::symbol => Sexpr::Symbol(pair.as_str().to_string()),
        Rule::list => Sexpr::List(pair.into_inner().map(parse_sexpr).collect()),
        _ => unreachable!(),
    }
}

pub fn parse_line(line: &str) -> Sexpr {
    let pair = LispParser::parse(Rule::sexpr, &line).expect("fail").next().unwrap();
    parse_sexpr(pair)
}

pub fn parse_code(file: &str) -> Vec<Sexpr> {
    use std::fs;
    let input = fs::read_to_string(file).expect("failed to read file");
    let code = LispParser::parse(Rule::program, &input).expect("fail to parse code").next().unwrap();

    code.into_inner()
        .map(parse_sexpr)
        .collect()
}