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
    fn parse_function(&self, toks: &mut Iter<String>) -> ForthResult<ForthFunc> {
        // Get the name
        if let Some(n) = toks.next() {
            let name = n.to_string(); // TODO: Check if the name is a valid one
            let mut definition: Vec<String> = vec![];

            while let Some(s) = toks.next() {
                if s == &";" {
                    // end of function definition
                    return Ok((name, definition));
                } else {
                    definition.push(s.to_string());
                }
            }
        }

        Err(format!("Invalid function"))
    }

    fn parse_string(&self, toks: &mut Iter<String>) -> ForthResult<String> {
        let mut msg = String::new();

        while let Some(m) = toks.next() {
            if m == "\"" {
                return Ok(msg);
            }
            if m.ends_with("\"") {
                let mut temp = m.clone();
                temp.pop();
                msg.push_str(&temp.clone());
                return Ok(msg);
            }
            msg.push_str(&format!("{} ", m));
        }

        Err(format!("Nonterminated string"))
    }

    fn eval_conditional(&self, env: &mut ForthEnv, toks: &mut Iter<String>) -> ForthResult<()> {
        let res = env.pop("Empty stack for condition in if".to_string())?;

        // Check the top of the stack and if it is not zero then evaluate till then
        let mut before_else = vec![];
        let mut after_else = vec![];
        let mut found_else = false;

        while let Some(t) = toks.next() {
            if t == "then" {
                break;
            }

            if t == "else" {
                found_else = true;
                continue;
            }

            if found_else {
                after_else.push(t.clone());
            } else {
                before_else.push(t.clone());
            }
        }

        if before_else.is_empty() {
            return Err("Empty statement for then clause".to_string());
        }
        if found_else && after_else.is_empty() {
            return Err("Empty statement for then clause after else".to_string());
        }

        if res == 0 {
            self.eval_toks(env, &mut before_else.iter());
        } else {
            self.eval_toks(env, &mut after_else.iter());
        }

        Ok(())
    }

    fn eval_special(
        &self,
        start: &str,
        env: &mut ForthEnv,
        toks: &mut Iter<String>,
    ) -> Option<ForthResult<()>> {
        // Handle function definitions
        if start == ":" {
            match self.parse_function(toks) {
                Ok(func) => {
                    println!("Defined: {:?}", func);
                    let fname = func.0.clone();
                    env.funcs.insert(fname, func);
                    return Some(Ok(()));
                }
                Err(e) => return Some(Err(e)),
            }
        }

        // Handle string output
        if start == ".\"" {
            match self.parse_string(toks) {
                Ok(msg) => {
                    print!("{}", msg);
                    return Some(Ok(()));
                }
                Err(e) => return Some(Err(e)),
            }
        }

        if start == "if" {
            match self.eval_conditional(env, toks) {
                Ok(()) => return Some(Ok(())),
                Err(e) => return Some(Err(e)),
            }
        }

        None
    }

    fn eval_toks(&self, env: &mut ForthEnv, toks: &mut Iter<String>) {
        while let Some(s) = toks.next() {
            if s.trim().is_empty() {
                // skip empty tokens
                continue;
            }

            // Handle special forms
            match self.eval_special(s, env, toks) {
                None => (),
                Some(Ok(())) => continue,
                Some(Err(e)) => {
                    println!("Error: {}", e);
                    break;
                }
            }

            // Handle as a number
            match s.parse::<i32>() {
                Ok(num) => env.push(num),
                Err(_) => {
                    if let Some(res) = self.builtin_operator(env, s) {
                        match res {
                            Err(e) => println!("Error: {}", e),
                            _ => (),
                        }
                    } else if env.funcs.contains_key(&s.to_string()) {
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
        let tokens: Vec<_> = expr.split(" ").map(|s| s.trim().to_string()).collect();

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
