use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyTuple};

fn main() -> PyResult<()> {
    Python::with_gil(quickstart)
}

fn base64_encode<'a>(py: Python<'a>) -> PyResult<()> {
    let func = py.import("quickstart")?.getattr("base64_encode")?;

    //let buf = "Hello Rust!";
    //let py_buf = unsafe { PyBytes::from_ptr(py, buf.as_ptr(), buf.len()) };
    let py_buf = str_to_py_bytes(py, "Hello Rust!");

    let args = PyTuple::new(py, &[py_buf]);
    let out: Vec<u8> = func.call1(args)?.extract()?;

    let s = String::from_utf8_lossy(out.as_slice());
    println!("out = '{}'", s);

    Ok(())
}

fn hello<'a>(py: Python<'a>) -> PyResult<()> {
    let quickstart = py.import("quickstart")?;
    quickstart.getattr("hello")?.call0().map(|_| ())
}

fn print_list_of_dict(py: Python<'_>) -> PyResult<()> {
    let func = py.import("quickstart")?.getattr("print_list_of_dict")?;

    // @TODO: figure out why PyList failed
    let list = vec![
        new_kv(py, "hello", "#1 Hello Rust!")?,
        new_kv(py, "world", "#2 Hello Rust!")?,
    ];

    let args = PyTuple::new(py, &[list]);
    func.call1(args)?;

    Ok(())
}

fn quickstart<'a>(py: Python<'a>) -> PyResult<()> {
    py.run(
        r#"
import sys;

sys.path.append('py');
"#,
        None,
        None,
    )?;

    hello(py.clone())?;
    base64_encode(py.clone())?;
    print_list_of_dict(py)?;

    Ok(())
}

// tools

fn new_kv<'a>(py: Python<'a>, k: &'static str, v: &'static str) -> PyResult<&'a PyDict> {
    let out = PyDict::new(py);

    out.set_item("key", str_to_py_bytes(py, k))?;
    out.set_item("value", str_to_py_bytes(py, v))?;

    Ok(out)
}

fn str_to_py_bytes<'a>(py: Python<'a>, s: &'static str) -> &'a PyBytes {
    unsafe { PyBytes::from_ptr(py, s.as_ptr(), s.len()) }
}
