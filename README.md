RustPython
==============

A Python interpreter written in Rust

# Installation

```
pip install byteplay
```

# Run

Given a python file `test.py`

```
python compile_code.py test.py > test.bytecode

cd RustPython
cargo run ../test.bytecode 
```

