use std::collections::HashMap;
use std::result;

use forth::ops;

pub type ForthResult<T> = result::Result<T, String>;
pub type Ops = Fn(&mut ForthEnv) -> ForthResult<()>;
pub type ForthFunc = (String, Vec<String>);

pub struct ForthEnv {
    pub stack: Vec<i32>,
    pub funcs: HashMap<String, ForthFunc>,
    pub vars: HashMap<String, i32>,
    pub var_refs: Vec<String>,
    pub constants: HashMap<String, i32>,
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

    pub fn print_vars(&self) {
        println!("{:?}", self.vars);
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
        self.builtins.insert("<".to_owned(), &ops::lt);
        self.builtins.insert(">".to_owned(), &ops::gt);
        self.builtins.insert("<=".to_owned(), &ops::lt_eq);
        self.builtins.insert(">=".to_owned(), &ops::gt_eq);
    }
}
