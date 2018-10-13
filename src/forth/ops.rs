use forth::inter::{ForthEnv, ForthResult};

type BinOp = fn(i32, i32) -> i32;

fn binary_op(name: &str, op: BinOp, env: &mut ForthEnv) -> ForthResult {
    let x = env
        .pop()
        .expect(&format!("Empty stack: for first argument for {}", name));
    let y = env
        .pop()
        .expect(&format!("Empty stack: for second argument for {}", name));
    env.push(op(x, y));
    Ok(())
}

pub fn print(env: &mut ForthEnv) -> ForthResult {
    print!("Stack: ");
    env.print();
    Ok(())
}

pub fn add(env: &mut ForthEnv) -> ForthResult {
    binary_op("+", |x, y| x + y, env)
}

pub fn subtract(env: &mut ForthEnv) -> ForthResult {
    binary_op("-", |x, y| y - x, env)
}

pub fn mul(env: &mut ForthEnv) -> ForthResult {
    binary_op("*", |x, y| x * y, env)
}

pub fn div(env: &mut ForthEnv) -> ForthResult {
    binary_op("/", |x, y| y / x, env)
}
