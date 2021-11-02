use crate::parser::*;
use std::collections::HashMap;


pub fn eval<'a>(env: &'a HashMap<String,Sexpr>, expr: &'a Sexpr) -> Sexpr {
    match expr {
        Sexpr::Symbol(s) => env.get(s).unwrap().clone(),
        // Sexpr::List(a) => Sexpr::List(a.iter().map(|elem| eval(env, elem)).collect()),
        Sexpr::List(a) => {
            let head = &a[0];
            let args = a[1..].iter().map(|elem| eval(env, elem)).collect();
            call(head, args)
        },
        val@_ => val.clone(),
    }
}

fn call(func: &Sexpr, args: Vec<Sexpr>) -> Sexpr {
    if let Sexpr::Symbol(s) = func{
        match s.as_str() {
            "+" => args[1..].iter().fold(args[0].clone(), |x,y| {
                match (x,y) {
                    (Sexpr::Number(a), Sexpr::Number(b)) => Sexpr::Number(a+b),
                    _ => unimplemented!(),
                }
            }),
            "-" => args[1..].iter().fold(args[0].clone(), |x,y| {
                match (x,y) {
                    (Sexpr::Number(a), Sexpr::Number(b)) => Sexpr::Number(a-b),
                    _ => unimplemented!(),
                }
            }),
            "*" => args[1..].iter().fold(args[0].clone(), |x,y| {
                match (x,y) {
                    (Sexpr::Number(a), Sexpr::Number(b)) => Sexpr::Number(a*b),
                    _ => unimplemented!(),
                }
            }),
            "/" => args[1..].iter().fold(args[0].clone(), |x,y| {
                match (x,y) {
                    (Sexpr::Number(a), Sexpr::Number(b)) => Sexpr::Number(a/b),
                    _ => unimplemented!(),
                }
            }),
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}
