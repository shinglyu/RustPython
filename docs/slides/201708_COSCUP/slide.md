class: center, middle
##Python Interpreter in Rust
###Introduction
#### 2017/3/28
#### Shing Lyu


???
top, middle, bottom
left, center, right

---
### Python's architecture
* Interpreted
* Garbage Collected
* Compiler => bytecode => VM

---
background-image: url('pic/ice-cream.jpg')
class: bleed
# Flavors


---
### Python Flavors
* CPython (THE python)
* Jython (JVM)
* IronPython (.NET)
* Pypy
* Educational
  * Byterun
  * Jsapy (JS)
  * Brython (Python in browser)

---
### Why rewriting Python in Rust?
* Memory safety

* Learn about Python internal
* Learn real world Rust

---
### Implementation strategy
* Focus on the VM first, then the compiler
  * Reuse the Python built-in compiler to generate bytecode
* Basic arithmetics
* Control flows (require JUMP)
* Function call (require call stack)
* Built-in functions (require native code)
* Run popular libraries


---
### References
* [`dis` documentation](https://docs.python.org/3.4/library/dis.html)
* [byterun](http://www.aosabook.org/en/500L/a-python-interpreter-written-in-python.html)
* [byterun (GitHub)](https://github.com/nedbat/byterun/)
* [cpython source code](https://github.com/python/cpython)

---
### How Python VM works
* Stack machine
* Accepts Python bytecode
* `python -m dis source.py`

---

### A simple Python code

```
#!/usr/bin/env python3
print(1+1)
```

We run `python3 -m dis source.py`

---

### The bytecode

```
  1    0 LOAD_NAME           0 (print)
       3 LOAD_CONST          2 (2)
       6 CALL_FUNCTION       1 (1 positional, 0 keyword pair)
       9 POP_TOP
      10 LOAD_CONST          1 (None)
      13 RETURN_VALUE

```

---

### LOAD_NAME "print"
* NAMES = ["print"]
* CONSTS = [None, 2]
* STACK:

```
 |                    |
 | print (native code)|
 +--------------------+
```
---
### LOAD_CONST 2
* NAMES = ["print"]
* CONSTS = [None, 2]
* STACK:

```
 |                    |
 |          2         |
 | print (native code)|
 +--------------------+
```

---

### CALL_FUNCTION 1
1. argument = stack.pop() (argument == 2)
2. function = stack.top() (function == print)
3. call print(2)

* NAMES = ["print"]
* CONSTS = [None, 2]
* STACK:

```
 |                    |
 | print (native code)|
 +--------------------+
```
---
### POP_TOP
* NAMES = ["print"]
* CONSTS = [None, 2]
* STACK:

```
 |                    |
 |     (empty)        |
 +--------------------+
```

---
### LOAD_CONST 1
* NAMES = ["print"]
* CONSTS = [None, 2]
* STACK:

```
 |                    |
 |       None         |
 +--------------------+
```

---
### RETURN_VALUE

(returns top of stack == None)

---

# Technical Detail

---

### Bytecode format
* `dis` output are for human
* Implementing a `dis` format parser is a waste of time
* Emit JSON bytecode using [bytecode](https://pypi.python.org/pypi/bytecode/0.5)

```
code = compile(f,...)  # Python built-in
bytecode.Bytecode().from_code(code).to_concrete_bytecode()  # Contains information we need
```

---

### Types
* Everything is a `PyObject` in CPython
* We'll need that class hierarchy eventually
* Use a Rusts `enum` for now
```
pub enum NativeType{
    NoneType,
    Boolean(bool),
    Int(i32),
    Float(f64),
    Str(String),
    Unicode(String),
    List(Vec<NativeType>),
    Tuple(Vec<NativeType>),
    Iter(Vec<NativeType>), // TODO: use Iterator instead
    Code(PyCodeObject),
    Function(Function),
    Slice(Option<i32>, Option<i32>, Option<i32>), // start, stop, step
    #[serde(skip_serializing, skip_deserializing)]
    NativeFunction(fn(Vec<NativeType>) -> NativeType ),
}
```

---

### Testing
* Need `assert` => Use `panic!()`

```
assert 1 == 1
```
```
  1           0 LOAD_CONST               0 (1)
              3 LOAD_CONST               0 (1)
              6 COMPARE_OP               2 (==)
              9 POP_JUMP_IF_TRUE        18
             12 LOAD_GLOBAL              0 (AssertionError)
             15 RAISE_VARARGS            1
        >>   18 LOAD_CONST               1 (None)
             21 RETURN_VALUE
```


---
# Not Covered
* Structural types
* Function call
* Builtin functions

---

### Next step
* Make it run a small but popular tool/library
* Implement the parser
* Figure out garbage collection
* Performance benchmarking


