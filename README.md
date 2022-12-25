# Decimal implementation as User Custom C API using PyO3

`pyo3_decimal` for c extention python class.

`pyo3_decimal_api` for pyo3 libraries to use `pyo3_decimal.Decimal`.

`pyo3_decimal_user` is an example of implementation.

``` rust
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
```
