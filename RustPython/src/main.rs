//extern crate eval; use eval::eval::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Debug, Clone)]
enum NativeType {
    None,
    Boolean(bool),
    Int(i32),
    Float(f32),
    String(String),
    Unicode(String),
}

const CMP_OP: &'static [&'static str] = &[">",
                                          "<=",
                                          "==",
                                          "!=",
                                          ">",
                                          ">=",
                                          "in",
                                          "not in",
                                          "is",
                                          "is not",
                                          "exception match",
                                          "BAD"
                                         ];
       

struct VirtualMachine<'a> {
    // TODO: We are using Option<i32> in stack for handline None return value
    // We need 1 stack per frame
    stack: Vec<NativeType>, 
    environment: HashMap<&'a str, NativeType>,
    labels: HashMap<usize, usize>, // Maps label id to line number, just for speedup
    lasti: usize, // Instruction location
    // cmp_op: Vec<&'a Fn(NativeType, NativeType) -> bool>, // TODO: change compare to a function list

}
impl<'a> VirtualMachine<'a> {
    fn new() -> VirtualMachine<'a> {
        VirtualMachine {
            stack: vec![],
            environment: HashMap::new(),
            labels: HashMap::new(),
            lasti: 0,
        }
    }
    // The Option<i32> is the return value of the frame, remove when we have implemented frame
    // TODO: read the op codes directly from the internal code object
    fn exec(&mut self, code: Code<'a>) -> NativeType {
        let mut ret = NativeType::None;

        for (line_no, op) in code.op_codes.iter().enumerate() {
            if let ("LABEL", Some(id)) = *op {
                self.labels.insert(id, line_no);
            }
        }

        // Change this to a loop for jump
        while self.lasti < code.op_codes.len() {
        let op_code = code.op_codes[self.lasti];
        self.lasti += 1; // Let's increment here, so if we use jump it will be overrided
        //for op in code.op_codes {
            // println!("Executing: {:?}", op);
            // TODO: convert this to enum?
            match op_code {
                ("LOAD_CONST", Some(consti)) => {
                    // println!("Loading const at index: {}", consti);
                    self.stack.push(code.consts[consti].clone());
                },
                // TODO: universal stack element type
                ("LOAD_CONST", None) => {
                    // println!("Loading const at index: {}", consti);
                    self.stack.push(NativeType::None);
                },
                ("STORE_NAME", Some(namei)) => {
                    // println!("Loading const at index: {}", consti);
                    self.environment.insert(code.names[namei], self.stack.pop().unwrap());
                },
                ("LOAD_NAME", Some(namei)) => {
                    // println!("Loading const at index: {}", consti);
                    self.stack.push(self.environment.get(code.names[namei]).unwrap().clone());
                },
                ("LOAD_GLOBAL", Some(namei)) => {
                    // We need to load the underlying value the name points to, but stuff like
                    // AssertionError is in the names right after compile, so we load the string
                    // instead for now
                    self.stack.push(NativeType::String(code.names[namei].to_string()));
                },
                ("COMPARE_OP", Some(cmp_op_i)) => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    match CMP_OP[cmp_op_i] {
                        // To avoid branch explotion, use an array of callables instead
                        "==" => {
                            match (v1, v2) {
                                (NativeType::Int(v1i), NativeType::Int(v2i)) => {
                                    self.stack.push(NativeType::Boolean(v2i == v1i));
                                },
                                (NativeType::Float(v1f), NativeType::Float(v2f)) => {
                                    self.stack.push(NativeType::Boolean(v2f == v1f));
                                },
                                _ => panic!("TypeError in COMPARE_OP")
                            };
                        }
                        _ => panic!("Unimplemented COMPARE_OP operator")

                    }
                    
                },
                ("POP_JUMP_IF_TRUE", Some(ref label_id)) => {
                    let v = self.stack.pop().unwrap();
                    if v == NativeType::Boolean(true) {
                        self.lasti = self.labels.get(label_id).unwrap().clone();
                    }

                }
                ("POP_JUMP_IF_FALSE", Some(ref label_id)) => {
                    let v = self.stack.pop().unwrap();
                    if v == NativeType::Boolean(false) {
                        self.lasti = self.labels.get(label_id).unwrap().clone();
                    }
                    
                }
                ("JUMP_FORWARD", Some(ref label_id)) => {
                    self.lasti = self.labels.get(label_id).unwrap().clone();
                    
                }
                ("RAISE_VARARGS", Some(argc)) => {
                    // let (exception, params, traceback) = match argc {
                    let exception = match argc {
                        1 => self.stack.pop().unwrap(),
                        0 | 2 | 3 => panic!("Not implemented!"),
                        _ => panic!("Invalid paramter for RAISE_VARARGS, must be between 0 to 3")
                    };
                    panic!("{:?}", exception);
                }

                ("BINARY_ADD", None) => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    match (v1, v2) {
                        (NativeType::Int(v1i), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Int(v2i + v1i));
                        }
                        (NativeType::Float(v1f), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Float(v2i as f32 + v1f));
                        } 
                        (NativeType::Int(v1i), NativeType::Float(v2f)) => {
                            self.stack.push(NativeType::Float(v2f + v1i as f32));
                        }
                        (NativeType::Float(v1f), NativeType::Float(v2f)) => {
                            self.stack.push(NativeType::Float(v2f + v1f));
                        }
                        (NativeType::String(str1), NativeType::String(str2)) => {
                            self.stack.push(NativeType::String(format!("{}{}", str2, str1)));

                        }
                        _ => panic!("TypeError in BINARY_ADD")
                    }
                },
                ("BINARY_POWER", None) => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    match (v1, v2) {
                        (NativeType::Int(v1i), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Int(v2i.pow(v1i as u32)));
                        }
                        (NativeType::Float(v1f), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Float((v2i as f32).powf(v1f)));
                        } 
                        (NativeType::Int(v1i), NativeType::Float(v2f)) => {
                            self.stack.push(NativeType::Float(v2f.powi(v1i)));
                        }
                        (NativeType::Float(v1f), NativeType::Float(v2f)) => {
                            self.stack.push(NativeType::Float(v2f.powf(v1f)));
                        }
                        _ => panic!("TypeError in BINARY_POWER")
                    }
                },
                ("BINARY_MULTIPLY", None) => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    match (v1, v2) {
                        (NativeType::Int(v1i), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Int(v2i * v1i));
                        },
                        /*
                        (NativeType::Float(v1f), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Float((v2i as f32) * v1f));
                        },
                        (NativeType::Int(v1i), NativeType::Float(v2f)) => {
                            self.stack.push(NativeType::Float(v2f * (v1i as f32)));
                        },
                        (NativeType::Float(v1f), NativeType::Float(v2f)) => {
                            self.stack.push(NativeType::Float(v2f * v1f));
                        },
                        */
                        //TODO: String multiply
                        _ => panic!("TypeError in BINARY_MULTIPLY")
                    }
                },
                ("BINARY_DIVIDE", None) => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    match (v1, v2) {
                        (NativeType::Int(v1i), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Int(v2i / v1i));
                        },
                        _ => panic!("TypeError in BINARY_DIVIDE")
                    }
                },
                ("BINARY_MODULO", None) => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    match (v1, v2) {
                        (NativeType::Int(v1i), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Int(v2i % v1i));
                        },
                        _ => panic!("TypeError in BINARY_MODULO")
                    }
                },
                ("BINARY_SUBTRACT", None) => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    match (v1, v2) {
                        (NativeType::Int(v1i), NativeType::Int(v2i)) => {
                            self.stack.push(NativeType::Int(v2i - v1i));
                        },
                        _ => panic!("TypeError in BINARY_SUBSTRACT")
                    }
                },
                ("UNARY_NEGATIVE", None) => {
                    let v = self.stack.pop().unwrap();
                    match v {
                        NativeType::Int(v1i) => {
                            self.stack.push(NativeType::Int(-v1i));
                        },
                        _ => panic!("TypeError in UINARY_NEGATIVE")
                    }
                },
                ("UNARY_POSITIVE", None) => {
                    let v = self.stack.pop().unwrap();
                    // Any case that is not just push back?
                    self.stack.push(v);
                },
                ("PRINT_ITEM", None) => {
                    // TODO: Print without the (...)
                    println!("{:?}", self.stack.pop().unwrap())
                },
                ("PRINT_NEWLINE", None) => {
                    print!("\n");
                },
                ("RETURN_VALUE", None) => {
                    ret = self.stack.pop().unwrap();
                    break;
                },
                ("SetLineno", _) | ("LABEL", _)=> {
                    // Skip
                }
                (name, _) => {
                    println!("Unrecongnizable op code: {}", name);
                }
            }
            // println!("Stack: {:?}", self.stack);
        }
        ret
    }
}

// The derive is temporarily disabled for testing native type
#[derive(PartialEq, Debug)]
struct Code<'a> {
    consts: Vec<NativeType>,
    names: Vec<&'a str>,
    op_codes: Vec<(&'a str, Option<usize>)>
}

fn parse_native_type(val_str: &str) -> Result<NativeType, ()> {
    // println!("{:?}", val_str);
    match val_str {
        "None" => Ok(NativeType::None),
        "True" => Ok(NativeType::Boolean(true)),
        "False" => Ok(NativeType::Boolean(false)),
        _ => {
            if let Ok(int) = val_str.parse::<i32>() {
                return Ok(NativeType::Int(int))
            }

            if let Ok(float) = val_str.parse::<f32>() {
                return Ok(NativeType::Float(float))
            }

            if val_str.starts_with("\'") && val_str.ends_with("\'") {
                return Ok(NativeType::String(val_str[1..val_str.len()-1].to_string()))
            }

            if val_str.starts_with("u\'") && val_str.ends_with("\'") {
                return Ok(NativeType::Unicode(val_str[2..val_str.len()-1].to_string()))
            }

            Err(())
        }

    }
}

fn parse_bytecode(s: &str) -> Code {
    let lines: Vec<&str> = s.split('\n').collect();

    let (metadata, ops) = lines.split_at(2);
    // Parsing the first line CONSTS
    let consts_str: &str = metadata[0]; // line 0 is empty
    let values_str = &consts_str[("CONSTS: (".len())..(consts_str.len()-1)];
    let values: Vec<&str> = values_str.split(",").collect();
    // We need better type definition here
    let consts: Vec<NativeType>= values.into_iter()
                                       .map(|x| x.trim())
                                       .filter(|x| x.len() > 0)
                                       .map(|x| parse_native_type(x).unwrap())
                                       .collect();

    // Parsing the second line NAMES
    let names_str: &str = metadata[1]; // line 0 is empty
    let values_str = &names_str[("NAMES: (".len())..(names_str.len()-1)];
    let values: Vec<&str> = values_str.split(",").collect();
    // We are assuming the first and last chars are \'
    let names: Vec<&str>= values.into_iter().map(|x| x.trim())
                                       .filter(|x| x.len() > 0)
        .map(|x| &x[1..(x.len()-1)]).collect();

    // Parsing the op_codes
    let op_codes: Vec<(&str, Option<usize>)>= ops.into_iter()
                                               .map(|x| x.trim())
                                               .filter(|x| x.len() > 0)
                                               .map(|x| {
                                                   let op: Vec<&str> = x.split(", ").collect();
                                                   // println!("{:?}", op);
                                                   (op[0], op[1].parse::<usize>().ok())
                                               }).collect();
    

    Code {
        consts: consts,
        op_codes: op_codes,
        names: names,
    }
}

fn main() {
    // TODO: read this from args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).unwrap();
    // println!("Read file");
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    // println!("Read string");
    let code = parse_bytecode(&s);
    // println!("Read code");

    let mut vm = VirtualMachine::new();
    vm.exec(code);
    // println!("Done");
}

#[test]
fn test_parse_native_type() {

    assert_eq!(NativeType::None, parse_native_type("None").unwrap());
    assert_eq!(NativeType::Boolean(true), parse_native_type("True").unwrap());
    assert_eq!(NativeType::Boolean(false), parse_native_type("False").unwrap());
    assert_eq!(NativeType::Int(3), parse_native_type("3").unwrap());
    assert_eq!(NativeType::Float(3.0), parse_native_type("3.0").unwrap());
    assert_eq!(NativeType::Float(3.5), parse_native_type("3.5").unwrap());
    assert_eq!(NativeType::String("foo".to_string()), parse_native_type("\'foo\'").unwrap());
    assert_eq!(NativeType::Unicode("foo".to_string()), parse_native_type("u\'foo\'").unwrap());
}

#[test]
fn test_parse_bytecode() {

    let input = "CONSTS: (1, None, 2)
NAMES: ('a', 'b')
SetLineno, 1
LOAD_CONST, 2
PRINT_ITEM, None
PRINT_NEWLINE, None
LOAD_CONST, None
RETURN_VALUE, None
        ";
    let expected = Code { // Fill me with a more sensible data
        consts: vec![NativeType::Int(1), NativeType::None, NativeType::Int(2)], 
        names: vec!["a", "b"],
        op_codes: vec![
            ("SetLineno", Some(1)),
            ("LOAD_CONST", Some(2)),
            ("PRINT_ITEM", None),
            ("PRINT_NEWLINE", None),
            ("LOAD_CONST", None),
            ("RETURN_VALUE", None)
        ]
    };

    assert_eq!(expected, parse_bytecode(input));
}

#[test]
fn test_single_const_tuple() {
    let input = "CONSTS: (None,)
NAMES: ()
SetLineno, 1
LOAD_CONST, 0
RETURN_VALUE, None
";
    let expected = Code { // Fill me with a more sensible data
        consts: vec![NativeType::None], 
        names: vec![],
        op_codes: vec![
            ("SetLineno", Some(1)),
            ("LOAD_CONST", Some(0)),
            ("RETURN_VALUE", None)
        ]
    };

    assert_eq!(expected, parse_bytecode(input));
}

#[test]
fn test_vm() {

    let code = Code {
        consts: vec![NativeType::Int(1), NativeType::None, NativeType::Int(2)], 
        names: vec![],
        op_codes: vec![
            ("LOAD_CONST", Some(2)),
            ("PRINT_ITEM", None),
            ("PRINT_NEWLINE", None),
            ("LOAD_CONST", None),
            ("RETURN_VALUE", None)
        ]
    };
    let mut vm = VirtualMachine::new();
    assert_eq!(NativeType::None, vm.exec(code));
}
