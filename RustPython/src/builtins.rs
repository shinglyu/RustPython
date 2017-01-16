use NativeType;

pub fn print(args: Vec<NativeType>) -> NativeType {
    for elem in args {
        // TODO: figure out how python's print vectors
        println!("{:?}", elem);
    }
    NativeType::NoneType
}
