pub mod eval {
    pub fn exec(bytecodes: Vec<String>){
        for op in bytecodes {
            println!("{:?}", op);
        }
    }
}
