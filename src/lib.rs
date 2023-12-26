use pyo3::prelude::*;
use ulid::{Ulid};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn test() -> PyResult<String> {
    Ok(Ulid::new().to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pkpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}
