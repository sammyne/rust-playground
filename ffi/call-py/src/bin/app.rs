use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyInt, PyList, PyTuple};

fn main() -> PyResult<()> {
    Python::with_gil(quickstart)
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

    let app_class = py.import("app")?.getattr("App")?;
    let app = app_class.call1(("hello world from rust",))?;

    let args = {
        let list = vec![
            new_kv(py, "hello", "#1 Hello Rust!")?,
            new_kv(py, "world", "#2 Hello Rust!")?,
        ];
        PyTuple::new(py, &[list])
    };

    let raw = app.call_method1("hello_world", args)?;
    let out = raw.downcast::<PyDict>()?;
    if let Some(v) = out.get_item("err") {
        let err: i64 = v.extract()?;
        if err != 0 {
            return Err(PyValueError::new_err(format!("non-zero error: {}", err)));
        }
    }

    let data = out.get_item("data").unwrap().downcast::<PyList>()?;
    for (i, v) in data.iter().enumerate() {
        let w = v.downcast::<PyBytes>().map(|v| v.as_bytes().to_vec())?;
        let s = String::from_utf8_lossy(&w);
        println!("out[{}] = {}", i, s);
    }

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
