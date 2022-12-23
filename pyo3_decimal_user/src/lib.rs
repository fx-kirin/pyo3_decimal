use pyo3;
use pyo3::prelude::*;
use pyo3_decimal_api::PyDecimal;

#[pyfunction]
/// Formats the sum of two numbers as string
fn decimal_test(a: PyDecimal) -> PyResult<PyDecimal> {
    Ok(a)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rust_binding(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(decimal_test))?;

    Ok(())
}
