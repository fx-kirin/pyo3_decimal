use pyo3;
use pyo3::prelude::*;
use pyo3_decimal_api::PyDecimal;
use rust_decimal::Decimal;

#[pyfunction]
/// Formats the sum of two numbers as string
fn decimal_test(a: PyDecimal) -> PyResult<PyDecimal> {
    Ok(a)
}

#[pyfunction]
/// Formats the sum of two numbers as string
fn cast_decimal(a: &mut PyDecimal) -> PyResult<&mut PyDecimal> {
    a.0 = a.0 + Decimal::new(1, 0);
    Ok(a)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rust_binding(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(decimal_test))?;

    Ok(())
}
