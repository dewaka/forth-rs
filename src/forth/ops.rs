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

pub fn modulus(env: &mut ForthEnv) -> ForthResult<()> {
    binary_op("mod", |x, y| y % x, env)
}

// Core operations
pub fn dup(env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack for dup"))?;
    env.push(x);
    env.push(x);
    Ok(())
}

pub fn pop(env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack for ."))?;
    println!("{}", x);
    Ok(())
}

pub fn swap(env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack for first element in swap"))?;
    let y = env.pop(format!("Empty stack for second element in swap"))?;
    env.push(x);
    env.push(y);
    Ok(())
}

pub fn over(env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack for first element in over"))?;
    let y = env.pop(format!("Empty stack for second element in over"))?;
    env.push(y);
    env.push(x);
    env.push(y);
    Ok(())
}

pub fn rot(env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack for first element in rot"))?;
    let y = env.pop(format!("Empty stack for second element in rot"))?;
    let z = env.pop(format!("Empty stack for third element in rot"))?;
    env.push(y);
    env.push(x);
    env.push(z);
    Ok(())
}

pub fn drop(env: &mut ForthEnv) -> ForthResult<()> {
    env.pop(format!("Empty stack for drop"))?;
    Ok(())
}

pub fn emit(env: &mut ForthEnv) -> ForthResult<()> {
    let x = env.pop(format!("Empty stack for emit"))?;
    print!("{}", (x as u8) as char);
    Ok(())
}

pub fn cr(_: &mut ForthEnv) -> ForthResult<()> {
    println!("");
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

pub fn print_vars(env: &mut ForthEnv) -> ForthResult<()> {
    print!("Variables: ");
    env.print_vars();
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

pub fn not_eq(env: &mut ForthEnv) -> ForthResult<()> {
    binary_bool_op("=", |x, y| x != y, env)
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
