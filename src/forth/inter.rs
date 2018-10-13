use std::collections::HashMap;
use std::result;
use std::slice::Iter;

use forth::ops;

pub type ForthResult<T> = result::Result<T, String>;
pub type Ops = Fn(&mut ForthEnv) -> ForthResult<()>;
pub type ForthFunc = (String, Vec<String>);

pub struct ForthEnv {
    stack: Vec<i32>,
    funcs: HashMap<String, ForthFunc>,
}

pub struct Interpreter<'a> {
    builtins: HashMap<String, &'a Ops>,
}

impl ForthEnv {
    pub fn empty() -> ForthEnv {
        ForthEnv {
            stack: vec![],
            funcs: HashMap::new(),
        }
    }

    pub fn pop(&mut self, msg: String) -> ForthResult<i32> {
        match self.stack.pop() {
            Some(n) => Ok(n),
            None => Err(msg),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    pub fn print_stack(&self) {
        println!("{:?}", self.stack);
    }

    pub fn print_func(&self) {
        println!("{:?}", self.funcs);
    }
}

impl<'a> Interpreter<'a> {
    fn parse_function(&self, toks: &mut Iter<String>) -> Option<ForthFunc> {
        // Get the name
        if let Some(n) = toks.next() {
            let name = n.to_string(); // TODO: Check if the name is a valid one
            let mut definition: Vec<String> = vec![];

            while let Some(s) = toks.next() {
                if s == &";" {
                    // end of function definition
                    return Some((name, definition));
                } else {
                    definition.push(s.to_string());
                }
            }
        }

        None
    }

    fn eval_toks(&self, env: &mut ForthEnv, toks: &mut Iter<String>) {
        while let Some(s) = toks.next() {
            if s == &":" {
                println!("Parsing function");
                match self.parse_function(toks) {
                    None => {
                        println!("Error parsing function");
                        return;
                    }
                    Some(func) => {
                        println!("Adding function: {:?}", func);
                        let fname = func.0.clone();
                        env.funcs.insert(fname, func);
                    }
                }

                continue;
            }

            match s.parse::<i32>() {
                Ok(num) => env.push(num),
                Err(_) => {
                    if let Some(res) = self.builtin_operator(env, s) {
                        match res {
                            Err(e) => println!("Error: {}", e),
                            _ => (),
                        }
                    } else if env.funcs.contains_key(&s.to_string()) {
                        println!("Applying function: {}", s);

                        let func = env.funcs.get(&s.to_string()).unwrap().clone();
                        self.eval_toks(env, &mut func.1.iter())
                    } else {
                        println!("Invalid token: {}", s);
                        break;
                    }
                }
            }
        }

        print!("=> ");
        env.print_stack();
    }

    pub fn eval(&self, env: &mut ForthEnv, expr: &str) {
        let tokens: Vec<_> = expr
            .split(" ")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        self.eval_toks(env, &mut tokens.iter());
    }

    fn builtin_operator(&self, env: &mut ForthEnv, op: &str) -> Option<ForthResult<()>> {
        if self.builtins.contains_key(op) {
            let opr = self.builtins.get(op).unwrap();
            Some(opr(env))
        } else {
            None
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
        // Binary ops
        self.builtins.insert("+".to_owned(), &ops::add);
        self.builtins.insert("-".to_owned(), &ops::subtract);
        self.builtins.insert("*".to_owned(), &ops::mul);
        self.builtins.insert("/".to_owned(), &ops::div);

        // Core ops
        self.builtins.insert("p".to_owned(), &ops::print_stack);
        self.builtins.insert("d".to_owned(), &ops::print_func);
        self.builtins.insert("dup".to_owned(), &ops::dup);
        self.builtins.insert(".".to_owned(), &ops::pop);

        // Boolean ops
        self.builtins.insert("=".to_owned(), &ops::eq);
        self.builtins.insert("<".to_owned(), &ops::lt);
        self.builtins.insert(">".to_owned(), &ops::gt);
        self.builtins.insert("<=".to_owned(), &ops::lt_eq);
        self.builtins.insert(">=".to_owned(), &ops::gt_eq);
    }
}
