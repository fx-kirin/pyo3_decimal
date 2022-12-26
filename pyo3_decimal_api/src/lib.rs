use pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::{ffi, PyResult, Python};
use rust_decimal::Decimal;
use std::cell::UnsafeCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;

static PYO3_CAPSULE_API_NAME: &std::ffi::CStr =
    unsafe { std::mem::transmute::<_, &std::ffi::CStr>(concat!("pyo3_decimal._API", "\0")) };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct PyO3Decimal_CAPI {
    pub DecimalType: *mut ffi::PyTypeObject,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PyDecimal(pub Decimal);

unsafe impl pyo3::type_object::PyTypeInfo for PyDecimal {
    type AsRefTarget = pyo3::PyCell<Self>;
    const NAME: &'static str = "Decimal";
    const MODULE: ::std::option::Option<&'static str> = Some("pyo3_decimal");
    #[inline]
    fn type_object_raw(py: pyo3::Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
        ensure_decimal_api(py).DecimalType
    }
}

impl pyo3::impl_::pyclass::PyClassImpl for PyDecimal {
    const DOC: &'static str = "\u{0}";
    const IS_BASETYPE: bool = false;
    const IS_SUBCLASS: bool = false;
    const IS_MAPPING: bool = false;
    type Layout = pyo3::PyCell<Self>;
    type BaseType = pyo3::PyAny;
    type ThreadChecker = pyo3::impl_::pyclass::ThreadCheckerStub<PyDecimal>;
    fn for_all_items(visitor: &mut dyn ::std::ops::FnMut(&pyo3::impl_::pyclass::PyClassItems)) {
        use pyo3::impl_::pyclass::*;
        let collector = PyClassImplCollector::<Self>::new();
        static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
            methods: &[],
            slots: &[],
        };
        visitor(&INTRINSIC_ITEMS);
        visitor(collector.py_methods());
        visitor(collector.object_protocol_items());
        visitor(collector.number_protocol_items());
        visitor(collector.iter_protocol_items());
        visitor(collector.gc_protocol_items());
        visitor(collector.descr_protocol_items());
        visitor(collector.mapping_protocol_items());
        visitor(collector.sequence_protocol_items());
        visitor(collector.async_protocol_items());
        visitor(collector.buffer_protocol_items());
    }
}

impl pyo3::PyClass for PyDecimal {
    type Dict = pyo3::impl_::pyclass::PyClassDummySlot;
    type WeakRef = pyo3::impl_::pyclass::PyClassDummySlot;
    type BaseNativeType = pyo3::PyAny;
}

impl<'a> pyo3::derive_utils::ExtractExt<'a> for &'a PyDecimal {
    type Target = pyo3::PyRef<'a, PyDecimal>;
}
impl<'a> pyo3::derive_utils::ExtractExt<'a> for &'a mut PyDecimal {
    type Target = pyo3::PyRefMut<'a, PyDecimal>;
}
struct Wrapper(PyCell<PyDecimal>);

unsafe impl pyo3::PyNativeType for Wrapper {}

fn ensure_decimal_api(_py: Python<'_>) -> &'static PyO3Decimal_CAPI {
    unsafe {
        if PyDecimalAPI().is_null() {
            PyDecimal_IMPORT();
        }

        &*PyDecimalAPI()
    }
}

#[inline]
/// Check if `op` is a `PyDateTimeAPI.DateTimeType` or subtype.
unsafe fn PyDecimal_Check(op: *mut ffi::PyObject) -> c_int {
    ffi::PyObject_TypeCheck(
        op,
        ensure_decimal_api(Python::assume_gil_acquired()).DecimalType,
    ) as c_int
}

impl pyo3::IntoPy<pyo3::PyObject> for PyDecimal {
    fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self).unwrap(), py)
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

#[inline]
unsafe fn PyDecimalAPI() -> *mut PyO3Decimal_CAPI {
    *PyDecimalAPI_impl.0.get()
}

struct PyDecimalAPISingleton(UnsafeCell<*mut PyO3Decimal_CAPI>);
unsafe impl Sync for PyDecimalAPISingleton {}
static PyDecimalAPI_impl: PyDecimalAPISingleton =
    PyDecimalAPISingleton(UnsafeCell::new(ptr::null_mut()));

/// Populates the `PyDecimalAPI` object
unsafe fn PyDecimal_IMPORT() {
    let py_decimal_c_api = {
        let module = CString::new("pyo3_decimal").unwrap();
        let capsule = CString::new("_API").unwrap();
        unsafe {
            let module = ffi::PyImport_ImportModule(module.as_ptr());
            assert!(!module.is_null(), "Failed to import pyo3_decimal module");
            let capsule = ffi::PyObject_GetAttrString(module as _, capsule.as_ptr());
            assert!(
                !capsule.is_null(),
                "Failed to get pyo3_decimal.Decimal API capsule"
            );
            ffi::PyCapsule_GetPointer(capsule, PYO3_CAPSULE_API_NAME.as_ptr())
                as *mut PyO3Decimal_CAPI
        }
    };
    *PyDecimalAPI_impl.0.get() = py_decimal_c_api;
}
