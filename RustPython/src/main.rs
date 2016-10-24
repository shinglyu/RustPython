extern crate eval;
use eval::eval::exec;

fn main() {
    println!("Hello, world!");
    exec(vec!["LOAD_FAST".to_string(), "FOO".to_string()]);
}
