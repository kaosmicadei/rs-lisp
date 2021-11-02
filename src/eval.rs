use crate::parser::*;
use std::collections::HashMap;

type Enviroment = HashMap<String,Sexpr>;

pub fn eval<'a>(env: &'a mut Enviroment, expr: &'a Sexpr) -> Sexpr {
    match expr {
        Sexpr::Symbol(s) => {
            match env.get(s) {
                val@Some(_) => val.unwrap().clone(),
                _ => Sexpr::Symbol(s.to_string()),
            }
        },
        Sexpr::List(a) => {
            if a.len() == 0 { return Sexpr::Bool(false) }
            let head = &a[0];
            let args = a[1..].iter().map(|elem| eval(env, elem)).collect();
            call(head, args, env)
        },
        val@_ => val.clone(),
    }
}

fn call(func: &Sexpr, args: Vec<Sexpr>, env: &mut Enviroment) -> Sexpr {
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
            "set" => {
                env.insert(args[0].to_string(), args[1].clone());
                args[1].clone()
            },
            "print" => {
                println!("{}", args[0]);
                Sexpr::Nil
            },
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}
