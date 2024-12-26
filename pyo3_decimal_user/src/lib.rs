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
fn cast_decimal(a: &mut PyDecimal) -> PyResult<PyDecimal> {
    a.0 = a.0 + Decimal::new(1, 0);
    Ok(a.0.into())
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rust_binding(m: &Bound<'_, PyModule>) -> PyResult<()>  {
    m.add_wrapped(wrap_pyfunction!(decimal_test))?;
    m.add_wrapped(wrap_pyfunction!(cast_decimal))?;

    Ok(())
}
