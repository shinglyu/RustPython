extern crate eval;
use eval::eval::*;

fn main() {
    println!("Hello, world!");
    let vm = VirtualMachine::new(vec![]);
    vm.exec(vec![("LOAD_FAST".to_string(), 2), ("FOO".to_string(), 1)]);
}
