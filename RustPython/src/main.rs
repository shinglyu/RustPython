//extern crate eval;
//use eval::eval::*;

struct VirtualMachine {
    consts: Vec<Option<i32>>, // Dynamic typing?
    // TODO: We are using Option<i32> in stack for handline None return value
    // We need 1 stack per frame
    stack: Vec<Option<i32>>, 
}

impl VirtualMachine {
    fn new(consts: Vec<Option<i32>>) -> VirtualMachine {
        VirtualMachine {
            consts: consts,
            stack: vec![]
        }
    }
    // The Option<i32> is the return value of the frame, remove when we have implemented frame
    fn exec(&mut self, ops: Vec<(&str, Option<i32>)>) -> Option<i32> {
        let mut ret = None;
        for op in ops {
            // println!("{:?}", op);
            // TODO: convert this to enum?
            match op {
                ("LOAD_CONST", Some(consti)) => {
                    // println!("Loading const at index: {}", consti);
                    self.stack.push(self.consts[consti as usize]);
                },
                // TODO: universal stack element type
                ("LOAD_CONST", None) => {
                    // println!("Loading const at index: {}", consti);
                    self.stack.push(None);
                },
                ("PRINT_ITEM", None) => {
                    // TODO: Print without the Some(...)
                    match self.stack.pop().unwrap() {
                        Some(x) => print!("{}", x),
                        None    => print!("None")
                    }
                },
                ("PRINT_NEWLINE", None) => {
                    print!("\n");
                },
                ("RETURN_VALUE", None) => {
                    ret = self.stack.pop().unwrap();
                    break;
                },
                _ => {
                    println!("Unrecongnized op code!");
                }
            }
        }
        ret
    }
}

fn main() {
    // TODO: read this from file
    let input = vec![
        ("LOAD_CONST", Some(2)),
        ("PRINT_ITEM", None),
        ("PRINT_NEWLINE", None),
        ("LOAD_CONST", None),
        ("RETURN_VALUE", None)
    ];

    // TODO: read this from file
    let consts = vec![Some(1), None, Some(2)];

    let mut vm = VirtualMachine::new(consts);
    vm.exec(input);
}

#[test]
fn test_vm() {

    let input = vec![
        ("LOAD_CONST", Some(2)),
        ("PRINT_ITEM", None),
        ("PRINT_NEWLINE", None),
        ("LOAD_CONST", None),
        ("RETURN_VALUE", None)
    ];

    let mut vm = VirtualMachine::new(vec![Some(1), None, Some(2)]);
    assert_eq!(None, vm.exec(input));
}
