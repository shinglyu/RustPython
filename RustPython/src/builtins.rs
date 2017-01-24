use NativeType;

pub fn print(args: Vec<NativeType>) -> NativeType {
    for elem in args {
        // TODO: figure out how python's print vectors
        match elem {
            NativeType::NoneType => println!("None"),
            NativeType::Boolean(b)=> {
                if b {
                    println!("True");
                } else {
                    println!("False");
                }
            },
            NativeType::Int(x)  => println!("{}", x),
            NativeType::Float(x)  => println!("{}", x),
            NativeType::Str(x)  => println!("{}", x),
            NativeType::Unicode(x)  => println!("{}", x),
            _ => panic!("Print for {:?} not implemented yet", elem),
            /*
            List(Vec<NativeType>),
            Tuple(Vec<NativeType>),
            Iter(Vec<NativeType>), // TODO: use Iterator instead
            Code(PyCodeObject),
            Function(Function),
            #[serde(skip_serializing, skip_deserializing)]
            NativeFunction(fn(Vec<NativeType>) -> NativeType ),
            */
        }
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
