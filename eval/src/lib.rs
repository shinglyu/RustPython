pub mod eval {
    pub struct VirtualMachine {
        consts: Vec<Option<i32>>, // Dynamic typing?
        stack: Vec<i32>, // We need 1 stack per frame
    }

    impl VirtualMachine {
        pub fn new(consts: Vec<Option<i32>>) -> VirtualMachine {
            VirtualMachine {
                consts: consts,
                stack: vec![]
            }
        }
        pub fn exec(&self, ops: Vec<(String, i32)>) {
            for op in ops {
                println!("{:?}", op);
            }
        }
    }

    pub fn exec(bytecodes: Vec<String>){
        for op in bytecodes {
            println!("{:?}", op);
        }
    }
}
