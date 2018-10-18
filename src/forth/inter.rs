use std::collections::HashMap;
use std::result;

use forth::ops;

pub type ForthResult<T> = result::Result<T, String>;
pub type Ops = Fn(&mut ForthEnv) -> ForthResult<()>;
pub type ForthFunc = (String, Vec<String>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ForthVar {
    Var(i32),
    Array(Vec<i32>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum VarRef {
    Var(String),
    Array(String, i32),
}

pub struct ForthEnv {
    pub stack: Vec<i32>,
    pub funcs: HashMap<String, ForthFunc>,
    pub vars: HashMap<String, ForthVar>,
    pub var_refs: Vec<VarRef>,
    pub constants: HashMap<String, i32>,
    pub specials: HashMap<String, i32>,
}

pub struct Interpreter<'a> {
    pub builtins: HashMap<String, &'a Ops>,
}

impl ForthEnv {
    pub fn empty() -> ForthEnv {
        ForthEnv {
            stack: vec![],
            funcs: HashMap::new(),
            vars: HashMap::new(),
            var_refs: vec![],
            constants: HashMap::new(),
            specials: HashMap::new(),
        }
    }

    pub fn pop(&mut self, msg: String) -> ForthResult<i32> {
        match self.stack.pop() {
            Some(n) => Ok(n),
            None => Err(msg),
        }
    }

    pub fn top(&mut self, msg: String) -> ForthResult<i32> {
        match self.stack.len() {
            0 => Err(msg),
            n => Ok(self.stack[n - 1]),
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

    pub fn print_vars(&self) {
        println!("{:?}", self.vars);
    }

    pub fn get_special(&self, name: &str) -> Option<i32> {
        if self.specials.contains_key(name) {
            let val = *self.specials.get(name).unwrap();
            Some(val)
        } else {
            None
        }
    }

    pub fn set_special(&mut self, name: &str, value: i32) -> Option<i32> {
        self.specials.insert(name.to_string(), value)
    }

    pub fn clear_special(&mut self, name: &str) -> Option<i32> {
        self.specials.remove(name)
    }

    pub fn allot_array(&mut self, name: &str, length: i32) {
        let arr = ForthVar::Array(vec![0; length as usize]);
        self.vars.insert(name.to_string(), arr);
    }

    pub fn array_set(&mut self, name: &str, pos: i32, value: i32) -> ForthResult<()> {
        if self.vars.contains_key(name) {
            let var = self.vars.get(name).unwrap().clone();
            match var {
                ForthVar::Var(_) => Err(format!("Variable {} is not an array!", name)),
                ForthVar::Array(vec) => {
                    let mut new_vec = vec.clone();
                    if new_vec.len() > (pos as usize) && pos >= 0 {
                        new_vec[pos as usize] = value;
                        self.vars.insert(name.to_string(), ForthVar::Array(new_vec));
                        Ok(())
                    } else {
                        Err(format!(
                            "Cannot set at position: {} when array length is: {}",
                            pos,
                            new_vec.len()
                        ))
                    }
                }
            }
        } else {
            Err(format!("No such array variable: {}", name))
        }
    }
}

impl<'a> Interpreter<'a> {
    pub fn eval(&self, env: &mut ForthEnv, expr: &str) {
        let tokens: Vec<_> = expr.split(" ").map(|s| s.trim().to_string()).collect();
        self.eval_toks(env, &mut tokens.iter());
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
        self.builtins.insert("mod".to_owned(), &ops::modulus);
        self.builtins.insert("and".to_owned(), &ops::and);
        self.builtins.insert("or".to_owned(), &ops::or);

        // Core ops
        self.builtins.insert("p".to_owned(), &ops::print_stack);
        self.builtins.insert("d".to_owned(), &ops::print_func);
        self.builtins.insert("v".to_owned(), &ops::print_vars);
        self.builtins.insert("dup".to_owned(), &ops::dup);
        self.builtins.insert(".".to_owned(), &ops::pop);
        self.builtins.insert("drop".to_owned(), &ops::drop);
        self.builtins.insert("swap".to_owned(), &ops::swap);
        self.builtins.insert("over".to_owned(), &ops::over);
        self.builtins.insert("rot".to_owned(), &ops::rot);
        self.builtins.insert("emit".to_owned(), &ops::emit);
        self.builtins.insert("cr".to_owned(), &ops::cr);

        // Boolean ops
        self.builtins.insert("=".to_owned(), &ops::eq);
        self.builtins.insert("!=".to_owned(), &ops::not_eq);
        self.builtins.insert("<".to_owned(), &ops::lt);
        self.builtins.insert(">".to_owned(), &ops::gt);
        self.builtins.insert("<=".to_owned(), &ops::lt_eq);
        self.builtins.insert(">=".to_owned(), &ops::gt_eq);
        self.builtins.insert("invert".to_owned(), &ops::invert);
    }
}
