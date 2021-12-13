use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn hello_world<'a>(py: Python<'a>) -> PyResult<()> {
    let sys = py.import("sys")?;
    let version: String = sys.getattr("version")?.extract()?;

    let locals = [("os", py.import("os")?)].into_py_dict(py);
    let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
    let user: String = py.eval(code, None, Some(&locals))?.extract()?;

    println!("Hello {}, I'm Python {}", user, version);
    Ok(())
}

fn main() -> PyResult<()> {
    Python::with_gil(hello_world).unwrap();

    {
        let gil = Python::acquire_gil();
        hello_world(gil.python())
    }
}
