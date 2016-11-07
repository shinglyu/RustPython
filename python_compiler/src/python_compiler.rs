use cpython::Python;
use cpython::ObjectProtocol; //for call method
use cpython::PyObject;
use python27_sys::PyCodeObject;


pub fn compile() -> PyObject {
    let gil = Python::acquire_gil();
    let py = gil.python();

    /*
    let sys = py.import("sys").unwrap();
    let version: String = sys.get(py, "version").unwrap().extract(py).unwrap();

    let os = py.import("os").unwrap();
    let getenv = os.get(py, "getenv").unwrap();
    let user: String = getenv.call(py, ("USER",), None).unwrap().extract(py).unwrap();
    */
    let builtin = py.import("__builtin__").unwrap();
    let compile = builtin.get(py, "compile").unwrap();
    let code: PyObject = compile.call(py, ("1+1", "foo", "exec"), None).unwrap();
    code
}
