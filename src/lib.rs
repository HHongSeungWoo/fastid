use pyo3::prelude::*;
use ulid::{Ulid};

#[pyfunction]
#[pyo3(name="ulid")]
fn get_ulid() -> PyResult<String> {
    Ok(Ulid::new().to_string())
}

#[pymodule]
fn pkpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_ulid, m)?)?;
    Ok(())
}
