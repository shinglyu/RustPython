use NativeType;

pub fn print(args: Vec<NativeType>) -> NativeType {
    for elem in args {
        // TODO: figure out how python's print vectors
        println!("{:?}", elem);
    }
    NativeType::NoneType
}

pub fn len(args: Vec<NativeType>) -> NativeType {
    if args.len() != 1 {
        panic!("len(s) expects exactly one parameter");
    }
    let len = match &args[0] {
        &NativeType::List(ref l) => l.len(),
        &NativeType::Tuple(ref t) => t.len(),
        _ => panic!("TypeError: object of this type has no len()")
    };
    NativeType::Int(len as i32)
}
