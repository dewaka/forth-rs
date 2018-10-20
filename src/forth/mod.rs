pub mod env;
pub mod inter;
mod ops;

fn valid_forth_name(name: &str) -> bool {
    match name.parse::<i32>() {
        Ok(_) => false,
        Err(_) => true,
    }
}
