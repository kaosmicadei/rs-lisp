#[path = "../src/parser.rs"]
mod parser;
use parser::*;
use parser::Sexpr::*;

#[test]
fn parse_nil() {
    assert_eq!(parse_line("()"), Nil);
}

#[test]
fn parse_bool() {
    assert_eq!(parse_line("true"), Bool(true));
    assert_eq!(parse_line("false"), Bool(false));
}

#[test]
fn parse_number() {
    assert_eq!(parse_line("1"), Number(1.0));
    assert_eq!(parse_line("2.1"), Number(2.1));
    assert_eq!(parse_line("314e-2"), Number(3.14));
    assert_eq!(parse_line("0.168E+1"), Number(1.68));
}

#[test]
fn parse_text() {
    assert_eq!(parse_line("\"this is a  test\""), Text("this is a  test".to_string()));
}

#[test]
fn parse_symbol() {
    assert_eq!(parse_line("pi"), Symbol("pi".to_string()));
    assert_eq!(parse_line("my-var"), Symbol("my-var".to_string()));
    assert_eq!(parse_line("+1"), Symbol("+1".to_string()));
}

#[test]
fn parse_list() {
    let val = List(vec![
        Symbol("+".to_string()), 
        Symbol("x".to_string()), 
        Number(1.0)
    ]);

    assert_eq!(parse_line("(+ x \n 1)"), val);
}