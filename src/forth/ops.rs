use forth::inter::{ForthEnv, ForthResult};

// Binary operations
type BinOp = fn(i32, i32) -> i32;

fn binary_op(name: &str, op: BinOp, env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack: for first argument for {}", name))?;
    let y = env.pop(format!("Empty stack: for second argument for {}", name))?;
    env.push(op(x, y));
    Ok(())
}

pub fn add(env: &mut ForthEnv) -> ForthResult<()> {
    binary_op("+", |x, y| x + y, env)
}

pub fn subtract(env: &mut ForthEnv) -> ForthResult<()> {
    binary_op("-", |x, y| y - x, env)
}

pub fn mul(env: &mut ForthEnv) -> ForthResult<()> {
    binary_op("*", |x, y| x * y, env)
}

pub fn div(env: &mut ForthEnv) -> ForthResult<()> {
    binary_op("/", |x, y| y / x, env)
}

// Core operations
pub fn dup(env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack for dup"))?;
    env.push(x);
    env.push(x);
    Ok(())
}

pub fn pop(env: &mut ForthEnv) -> ForthResult<()> {
    env.pop(format!("Empty stack for ."))?;
    Ok(())
}

pub fn print_stack(env: &mut ForthEnv) -> ForthResult<()> {
    print!("Stack: ");
    env.print_stack();
    Ok(())
}

pub fn print_func(env: &mut ForthEnv) -> ForthResult<()> {
    print!("Dictionary: ");
    env.print_func();
    Ok(())
}

// Boolean operations
type BinBoolOp = fn(i32, i32) -> bool;

fn binary_bool_op(name: &str, op: BinBoolOp, env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack: for first argument for {}", name))?;
    let y = env.pop(format!("Empty stack: for second argument for {}", name))?;
    if op(x, y) {
        env.push(-1);
    } else {
        env.push(0);
    }
    Ok(())
}

pub fn eq(env: &mut ForthEnv) -> ForthResult<()> {
    binary_bool_op("=", |x, y| x == y, env)
}

pub fn lt(env: &mut ForthEnv) -> ForthResult<()> {
    binary_bool_op("<", |x, y| y < x, env)
}

pub fn gt(env: &mut ForthEnv) -> ForthResult<()> {
    binary_bool_op(">", |x, y| y > x, env)
}

pub fn lt_eq(env: &mut ForthEnv) -> ForthResult<()> {
    binary_bool_op("<=", |x, y| y <= x, env)
}

pub fn gt_eq(env: &mut ForthEnv) -> ForthResult<()> {
    binary_bool_op(">=", |x, y| y >= x, env)
}
