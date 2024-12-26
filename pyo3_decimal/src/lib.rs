use bincode;
use bincode::{deserialize, serialize};
use pyo3;
use pyo3::ffi::PyCapsule_New;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyCapsule;
use pyo3::{PyResult, PyTypeInfo, Python};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::os::raw::c_void;
use std::ops::Deref;

use pyo3::ffi::PyTypeObject;

static PYO3_CAPSULE_API_NAME: &std::ffi::CStr =
    unsafe { std::mem::transmute::<_, &std::ffi::CStr>(concat!("pyo3_decimal._API", "\0")) };
static mut PYO3_DECIMAL_CAPI: *const PyO3Decimal_CAPI = std::ptr::null();

#[pyclass(module = "pyo3_decimal", name = "Decimal")]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct PyDecimal(Decimal);

#[pymethods]
impl PyDecimal {
    #[new]
    #[pyo3(signature=(arg1, arg2 = None))]
    pub fn new<'p>(
        arg1: PyObject,
        arg2: Option<PyObject>,
        py: Python<'p>,
    ) -> pyo3::PyResult<PyDecimal> {
        let py_int = arg1.downcast_bound::<pyo3::types::PyInt>(py);
        if let Ok(content) = py_int {
            let num: i128 = content.extract().unwrap();
            let scale: u32 = if let Some(arg2) = arg2 {
                let py_int = arg2.downcast_bound::<pyo3::types::PyInt>(py);
                if let Ok(content) = py_int {
                    content.extract().unwrap()
                } else {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "arg2 is wrong value. {:?}",
                        arg2
                    )));
                }
            } else {
                0
            };
            return Ok(Self(Decimal::from_i128_with_scale(num, scale)))
        };
        let py_string = arg1.downcast_bound::<pyo3::types::PyString>(py);
        if let Ok(content) = py_string {
            let rust_str: &str = &content.to_str().unwrap();
            let result = rust_decimal::Decimal::from_str_exact(rust_str);
            if arg2.is_some() {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "arg1 is String but arg2 was supplied value. {:?}",
                    arg2
                )));
            }
            return match result {
                Ok(v) => Ok(Self(v)),
                Err(_) => Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "arg1 is wrong value. {}",
                    rust_str
                ))),
            };
        }
        let py_float = arg1.downcast_bound::<pyo3::types::PyFloat>(py);
        if let Ok(content) = py_float {
            let num: f64 = content.extract().unwrap();
            if arg2.is_some() {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "arg1 is Float but arg2 was supplied value. {:?}",
                    arg2
                )));
            }
            return Ok(Self(
                Decimal::from_f64_retain(num).expect("Failed to load from float value"),
            ));
        } else {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "arg1 is wrong value. {:?}",
                arg1
            )));
        }
    }

    pub const fn scale(&self) -> u32 {
        self.0.scale()
    }

    pub const fn mantissa(&self) -> i128 {
        self.0.mantissa()
    }

    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn set_sign_positive(&mut self, positive: bool) {
        self.0.set_sign_positive(positive)
    }

    //#[inline(always)]
    pub fn set_sign_negative(&mut self, negative: bool) {
        self.0.set_sign_negative(negative)
    }

    pub fn set_scale(&mut self, scale: u32) -> pyo3::PyResult<()> {
        let result = self.0.set_scale(scale);
        match result {
            Ok(v) => Ok(v),
            Err(_) => Err(pyo3::exceptions::PyRuntimeError::new_err("set_scale Error")),
        }
    }

    pub fn rescale(&mut self, scale: u32) {
        self.0.rescale(scale)
    }

    pub const fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    pub const fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    pub fn trunc(&self) -> PyDecimal {
        self.0.trunc().into()
    }

    pub fn fract(&self) -> PyDecimal {
        self.0.fract().into()
    }

    pub fn abs(&self) -> PyDecimal {
        self.0.abs().into()
    }

    pub fn floor(&self) -> PyDecimal {
        self.0.floor().into()
    }

    pub fn ceil(&self) -> PyDecimal {
        self.0.ceil().into()
    }

    pub fn max(&self, other: PyDecimal) -> PyDecimal {
        self.0.max(*other.deref()).into()
    }

    pub fn min(&self, other: PyDecimal) -> PyDecimal {
        self.0.min(*other.deref()).into()
    }

    pub fn normalize(&self) -> PyDecimal {
        self.0.normalize().into()
    }

    pub fn normalize_assign(&mut self) {
        self.0.normalize_assign()
    }

    pub fn round(&self) -> PyDecimal {
        self.0.round().into()
    }

    pub fn round_dp(&self, dp: u32) -> PyDecimal {
        self.0.round_dp(dp).into()
    }

    pub fn round_sf(&self, digits: u32) -> Option<PyDecimal> {
        let decimal = self.0.round_sf(digits);
        if decimal.is_some() {
            Some(decimal.unwrap().into())
        } else {
            None
        }
    }

    pub fn to_int(&self) -> i64 {
        self.0.to_i64().unwrap()
    }

    pub fn to_float(&self) -> f64 {
        self.0.to_f64().unwrap()
    }

    fn __add__(&self, other: &Bound<'_, PyDecimal>) -> pyo3::PyResult<PyDecimal> {
        Ok((self.0 + other.extract::<PyDecimal>().unwrap().0).into())
    }

    fn __sub__(&self, other: &Bound<'_, PyDecimal>) -> pyo3::PyResult<PyDecimal> {
        Ok((self.0 - other.extract::<PyDecimal>().unwrap().0).into())
    }

    fn __mul__(&self, other: &Bound<'_, PyDecimal>) -> pyo3::PyResult<PyDecimal> {
        Ok((self.0 * other.extract::<PyDecimal>().unwrap().0).into())
    }

    fn __truediv__(&self, other: &Bound<'_, PyDecimal>) -> pyo3::PyResult<PyDecimal> {
        Ok((self.0 / other.extract::<PyDecimal>().unwrap().0).into())
    }

    fn __floordiv__(&self, other: &Bound<'_, PyDecimal>) -> pyo3::PyResult<PyDecimal> {
        Ok((self.0 / other.extract::<PyDecimal>().unwrap().0).into())
    }

    fn __neg__(&self) -> pyo3::PyResult<PyDecimal> {
        Ok((-self.0).into())
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyDecimal>,
        op: pyo3::class::basic::CompareOp,
    ) -> pyo3::PyResult<bool> {
        match op {
            pyo3::class::basic::CompareOp::Lt => Ok(self.0 < other.extract::<PyDecimal>().unwrap().0),
            pyo3::class::basic::CompareOp::Le => Ok(self.0 <= other.extract::<PyDecimal>().unwrap().0),
            pyo3::class::basic::CompareOp::Eq => Ok(self.0 == other.extract::<PyDecimal>().unwrap().0),
            pyo3::class::basic::CompareOp::Ne => Ok(self.0 != other.extract::<PyDecimal>().unwrap().0),
            pyo3::class::basic::CompareOp::Gt => Ok(self.0 > other.extract::<PyDecimal>().unwrap().0),
            pyo3::class::basic::CompareOp::Ge => Ok(self.0 >= other.extract::<PyDecimal>().unwrap().0),
        }
    }

    fn __str__(&self) -> pyo3::PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> pyo3::PyResult<String> {
        Ok(format!("Decimal({})", self.to_string()))
    }

    fn __int__(&self) -> i64 {
        self.to_int()
    }

    fn __float__(&self) -> f64 {
        self.to_float()
    }

    fn __abs__(&self) -> pyo3::PyResult<PyDecimal> {
        Ok(self.0.abs().into())
    }

    fn __format__(&self, format_spec: &str) -> pyo3::PyResult<String> {
        let text_length = format_spec.len();
        if text_length == 0 {
            return Ok(self.to_string());
        }
        let format_base = &format_spec[text_length - 1..text_length];
        if format_base == "i" {
            if text_length == 1 {
                return Ok(self.to_int().to_string());
            }
            let format_prefix = &format_spec[0..(text_length - 1)];
            let result = num_runtime_fmt::NumFmt::from_str(&*format_prefix);
            let result = match result {
                Ok(r) => r,
                Err(e) => {
                    return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "format string error {}",
                        e.to_string()
                    )));
                }
            };
            let result = result.fmt(self.to_int());
            let result = match result {
                Ok(r) => r,
                Err(e) => {
                    return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "format string error {}",
                        e.to_string()
                    )));
                }
            };
            return Ok(result);
        } else if format_base == "f" {
            if text_length == 1 {
                return Ok(self.to_float().to_string());
            }
            let format_prefix = &format_spec[0..(text_length - 1)];
            let result = num_runtime_fmt::NumFmt::from_str(&*format_prefix);
            let result = match result {
                Ok(r) => r,
                Err(e) => {
                    return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "format string error {}",
                        e.to_string()
                    )));
                }
            };
            let result = result.fmt(self.to_float());
            let result = match result {
                Ok(r) => r,
                Err(e) => {
                    return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "format string error {}",
                        e.to_string()
                    )));
                }
            };
            return Ok(result);
        } else {
            let result = num_runtime_fmt::NumFmt::from_str(&*format_spec);
            let result = match result {
                Ok(r) => r,
                Err(e) => {
                    return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "format string error {}",
                        e.to_string()
                    )));
                }
            };
            let result = result.fmt(self.to_float());
            let result = match result {
                Ok(r) => r,
                Err(e) => {
                    return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "format string error {}",
                        e.to_string()
                    )));
                }
            };
            return Ok(result);
        }
    }
    // Pickle
    pub fn __setstate__(&mut self, state: &Bound<'_, PyBytes>) -> PyResult<()> {
        *self = deserialize(state.as_bytes()).unwrap();
        Ok(())
    }
    pub fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(py, &serialize(&self).unwrap()))
    }

    pub fn __getnewargs__(&self) -> PyResult<(i128, u32)> {
        Ok((self.0.mantissa(), self.0.scale()))
    }
}

impl std::ops::Deref for PyDecimal {
    type Target = Decimal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PyDecimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<Decimal> for PyDecimal {
    fn into(self) -> Decimal {
        self.0
    }
}

impl Into<PyDecimal> for Decimal {
    fn into(self) -> PyDecimal {
        PyDecimal(self)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyO3Decimal_CAPI {
    pub DecimalType: *mut PyTypeObject,
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pyo3_decimal(m: &Bound<'_, PyModule>) -> PyResult<()>  {
    Python::with_gil(|py| {
        let decimal_type = PyDecimal::type_object_raw(py);
        let mut _decimal_api = PyO3Decimal_CAPI {
            DecimalType: decimal_type,
        };
        let mut _decimal_api = Box::new(_decimal_api);
        unsafe {
            // leak the value, so it will never be dropped or freed
            PYO3_DECIMAL_CAPI = Box::leak(_decimal_api) as *const PyO3Decimal_CAPI;
        }
    });
    unsafe {
        let cap_ptr = PyCapsule_New(
            PYO3_DECIMAL_CAPI as *mut c_void,
            (*PYO3_CAPSULE_API_NAME).as_ptr(),
            None,
        );
        Python::with_gil(|py| {
            let capsule: &Bound<'_, PyAny> = &Bound::<'_, PyAny>::from_owned_ptr_or_err(py, cap_ptr).unwrap();
            m.add("_API", capsule).unwrap();
        })
    }

    m.add_class::<PyDecimal>()?;

    Ok(())
}
