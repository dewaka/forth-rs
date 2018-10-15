use forth::inter::{ForthEnv, ForthFunc, ForthResult, Interpreter};
use std::slice::Iter;

impl<'a> Interpreter<'a> {
    fn eval_builtin(&self, op: &str, env: &mut ForthEnv) -> Option<ForthResult<()>> {
        if self.builtins.contains_key(op) {
            let opr = self.builtins.get(op).unwrap();
            Some(opr(env))
        } else {
            None
        }
    }

    fn eval_function(&self, name: &str, env: &mut ForthEnv) -> Option<ForthResult<()>> {
        if env.funcs.contains_key(&name.to_string()) {
            let func = env.funcs.get(&name.to_string()).unwrap().clone();
            self.eval_toks(env, &mut func.1.iter());
            Some(Ok(()))
        } else {
            None
        }
    }

    fn eval_variable(&self, name: &str, env: &mut ForthEnv) -> Option<ForthResult<()>> {
        if env.vars.contains_key(&name.to_string()) {
            env.var_refs.push(name.to_string());
            Some(Ok(()))
        } else {
            None
        }
    }

    fn eval_constant(&self, name: &str, env: &mut ForthEnv) -> Option<ForthResult<()>> {
        if env.constants.contains_key(&name.to_string()) {
            let x = env.constants.get(&name.to_string()).unwrap().clone();
            env.push(x);
            Some(Ok(()))
        } else {
            None
        }
    }

    fn eval_special_variables(&self, name: &str, env: &mut ForthEnv) -> bool {
        match env.get_special(name) {
            Some(val) => {
                env.push(val);
                true
            }
            None => false,
        }
    }

    pub fn eval_toks(&self, env: &mut ForthEnv, toks: &mut Iter<String>) {
        while let Some(s) = toks.next() {
            if s.trim().is_empty() {
                // skip empty tokens
                continue;
            }

            // Handle special forms
            match self.eval_special_forms(s, env, toks) {
                None => (),
                Some(Ok(())) => continue,
                Some(Err(e)) => {
                    println!("Error: {}", e);
                    break;
                }
            }

            // Handle special variables
            if self.eval_special_variables(s, env) {
                continue;
            }

            match self.eval_builtin(s, env) {
                None => (),
                Some(Ok(())) => continue,
                Some(Err(e)) => {
                    println!("Error: {}", e);
                    break;
                }
            }

            match self.eval_function(s, env) {
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
                    // Check if this is a valid variable or not
                    match self.eval_constant(s, env) {
                        None => match self.eval_variable(s, env) {
                            None => {
                                println!("Invalid token: {}", s);
                                break;
                            }
                            Some(Ok(())) => continue,
                            Some(Err(e)) => {
                                println!("Error: {}", e);
                                break;
                            }
                        },
                        Some(Ok(())) => continue,
                        Some(Err(e)) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            }
        }

        // print!("=> ");
        // env.print_stack();
    }

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
            self.eval_toks(env, &mut after_else.iter());
        } else {
            self.eval_toks(env, &mut before_else.iter());
        }

        Ok(())
    }

    fn eval_special_forms(
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

        if start == "@" {
            // Variable get
            match self.eval_variable_get(env) {
                Ok(()) => return Some(Ok(())),
                Err(e) => return Some(Err(e)),
            }
        }

        if start == "!" {
            // Variable set
            match self.eval_variable_set(env) {
                Ok(()) => return Some(Ok(())),
                Err(e) => return Some(Err(e)),
            }
        }

        if start == "variable" {
            // Variable introduction
            match self.eval_intro_variable(env, toks) {
                Ok(()) => return Some(Ok(())),
                Err(e) => return Some(Err(e)),
            }
        }

        if start == "constant" {
            // Variable introduction
            match self.eval_intro_constant(env, toks) {
                Ok(()) => return Some(Ok(())),
                Err(e) => return Some(Err(e)),
            }
        }

        if start == "do" {
            // do loop
            match self.eval_do_loop(env, toks) {
                Ok(()) => return Some(Ok(())),
                Err(e) => return Some(Err(e)),
            }
        }

        None
    }

    fn eval_intro_variable(&self, env: &mut ForthEnv, toks: &mut Iter<String>) -> ForthResult<()> {
        if let Some(var_name) = toks.next() {
            // TODO: Check if this is a valid variable name or not
            env.vars.insert(var_name.clone(), 0);
            Ok(())
        } else {
            Err(format!("Variable name not found"))
        }
    }

    // do [loop body] loop
    fn eval_do_loop(&self, env: &mut ForthEnv, toks: &mut Iter<String>) -> ForthResult<()> {
        let mut loop_body = vec![];

        while let Some(t) = toks.next() {
            if t == "loop" {
                break;
            }
            loop_body.push(t.clone());
        }

        if loop_body.is_empty() {
            return Err(format!("Empty loop body"));
        }

        let start = env.pop(format!("Empty stack for start of do loop"))?;
        let end = env.pop(format!("Empty stack for end of do loop"))?;

        for i in start..end {
            env.set_special("i", i);
            self.eval_toks(env, &mut loop_body.iter());
        }

        env.clear_special("i");

        Ok(())
    }

    fn eval_intro_constant(&self, env: &mut ForthEnv, toks: &mut Iter<String>) -> ForthResult<()> {
        if let Some(const_name) = toks.next() {
            let x = env.pop(format!("Stack empty to set constant {}", const_name))?;
            env.constants.insert(const_name.clone(), x);
            Ok(())
        } else {
            Err(format!("Variable name not found"))
        }
    }

    fn eval_variable_set(&self, env: &mut ForthEnv) -> ForthResult<()> {
        match env.var_refs.pop() {
            Some(var_name) => {
                if env.vars.contains_key(&var_name) {
                    let x = env.pop(format!("Stack empty to set variable value"))?;
                    env.vars.insert(var_name, x);
                    Ok(())
                } else {
                    Err(format!("No such variable: {}", var_name))
                }
            }
            None => Err(format!("No variable reference found to set value")),
        }
    }

    fn eval_variable_get(&self, env: &mut ForthEnv) -> ForthResult<()> {
        match env.var_refs.pop() {
            Some(var_name) => match env.vars.get(&var_name) {
                Some(&value) => {
                    env.push(value);
                    Ok(())
                }
                None => Err(format!("No such variable: {}", var_name)),
            },
            None => Err(format!("No variable reference found to set value")),
        }
    }
}
