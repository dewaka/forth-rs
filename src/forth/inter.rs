use std::collections::HashMap;
use std::{error, result};

use forth::ops;

pub struct ForthEnv {
    stack: Vec<i32>,
    dict: HashMap<String, String>,
}

pub struct Interpreter<'a> {
    builtins: HashMap<String, &'a Ops>,
}

pub type ForthResult = result::Result<(), String>;

pub type Ops = Fn(&mut ForthEnv) -> ForthResult;

impl ForthEnv {
    pub fn empty() -> ForthEnv {
        ForthEnv {
            stack: vec![],
            dict: HashMap::new(),
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    pub fn print(&self) {
        println!("{:?}", self.stack);
    }
}

impl<'a> Interpreter<'a> {
    pub fn eval(&self, env: &mut ForthEnv, expr: &str) {
        let split = expr.split(" ");

        for s in split {
            match s.parse::<i32>() {
                Ok(num) => env.push(num),
                Err(_) => {
                    if let Err(e) = self.builtin_operator(env, s) {
                        println!("Error: {}", e);
                    }
                }
            }
        }

        print!("=> ");
        env.print();
    }

    fn builtin_operator(&self, env: &mut ForthEnv, op: &str) -> ForthResult {
        if self.builtins.contains_key(op) {
            let opr = self.builtins.get(op).unwrap();
            opr(env)
        } else {
            Err(format!("No such operator - {}", op))
        }
    }

    pub fn new() -> Self {
        let mut intr = Interpreter {
            builtins: HashMap::new(),
        };

        intr.init();
        intr
    }

    fn init(&mut self) {
        self.builtins.insert("p".to_owned(), &ops::print);
        self.builtins.insert("+".to_owned(), &ops::add);
        self.builtins.insert("-".to_owned(), &ops::subtract);
        self.builtins.insert("*".to_owned(), &ops::mul);
        self.builtins.insert("/".to_owned(), &ops::div);
    }
}
