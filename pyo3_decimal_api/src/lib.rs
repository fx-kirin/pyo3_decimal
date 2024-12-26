#![feature(c_str_literals)]
#![feature(derive_clone_copy)]
#![feature(fmt_helpers_for_derive)]
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

#[inline]
/// Check if `op` is a `PyDateTimeAPI.DateTimeType` or subtype.
pub unsafe fn PyDecimal_Check(op: *mut ffi::PyObject) -> c_int {
    ffi::PyObject_TypeCheck(
        op,
        ensure_decimal_api(Python::assume_gil_acquired()).DecimalType,
    ) as c_int
}

struct PyDecimalAPISingleton(UnsafeCell<*mut PyO3Decimal_CAPI>);
unsafe impl Sync for PyDecimalAPISingleton {}

static PyDecimalAPI_impl: PyDecimalAPISingleton =
    PyDecimalAPISingleton(UnsafeCell::new(ptr::null_mut()));

#[inline]
unsafe fn PyDecimalAPI() -> *mut PyO3Decimal_CAPI {
    *PyDecimalAPI_impl.0.get()
}

fn ensure_decimal_api(_py: Python<'_>) -> &'static PyO3Decimal_CAPI {
    unsafe {
        if PyDecimalAPI().is_null() {
            PyDecimal_IMPORT();
        }

        &*PyDecimalAPI()
    }
}

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

pub struct PyDecimal(pub Decimal);
impl ::pyo3::types::DerefToPyAny for PyDecimal {}
unsafe impl ::pyo3::type_object::PyTypeInfo for PyDecimal {
    const NAME: &'static str = "Decimal";
    const MODULE: ::std::option::Option<&'static str> =
        ::core::option::Option::Some("pyo3_decimal");
    #[inline]
    fn type_object_raw(py: ::pyo3::Python<'_>) -> *mut ::pyo3::ffi::PyTypeObject {
        ensure_decimal_api(py).DecimalType
    }
}
impl ::pyo3::PyClass for PyDecimal {
    type Frozen = ::pyo3::pyclass::boolean_struct::False;
}
impl<'a, 'py> ::pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a PyDecimal {
    type Holder = ::std::option::Option<::pyo3::PyRef<'py, PyDecimal>>;
    #[inline]
    fn extract(
        obj: &'a ::pyo3::Bound<'py, ::pyo3::PyAny>,
        holder: &'a mut Self::Holder,
    ) -> ::pyo3::PyResult<Self> {
        ::pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
    }
}
impl<'a, 'py> ::pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut PyDecimal {
    type Holder = ::std::option::Option<::pyo3::PyRefMut<'py, PyDecimal>>;
    #[inline]
    fn extract(
        obj: &'a ::pyo3::Bound<'py, ::pyo3::PyAny>,
        holder: &'a mut Self::Holder,
    ) -> ::pyo3::PyResult<Self> {
        ::pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
    }
}
#[allow(deprecated)]
impl ::pyo3::IntoPy<::pyo3::PyObject> for PyDecimal {
    fn into_py(self, py: ::pyo3::Python<'_>) -> ::pyo3::PyObject {
        ::pyo3::IntoPy::into_py(::pyo3::Py::new(py, self).unwrap(), py)
    }
}
impl<'py> ::pyo3::conversion::IntoPyObject<'py> for PyDecimal {
    type Target = Self;
    type Output = ::pyo3::Bound<'py, <Self as ::pyo3::conversion::IntoPyObject<'py>>::Target>;
    type Error = ::pyo3::PyErr;
    fn into_pyobject(
        self,
        py: ::pyo3::Python<'py>,
    ) -> ::std::result::Result<
        <Self as ::pyo3::conversion::IntoPyObject>::Output,
        <Self as ::pyo3::conversion::IntoPyObject>::Error,
    > {
        ::pyo3::Bound::new(py, self)
    }
}
const _: () = {
    use ::pyo3::impl_::pyclass::*;
    assert_pyclass_sync::<PyDecimal>();
};
impl ::pyo3::impl_::pyclass::PyClassImpl for PyDecimal {
    const IS_BASETYPE: bool = false;
    const IS_SUBCLASS: bool = false;
    const IS_MAPPING: bool = false;
    const IS_SEQUENCE: bool = false;
    type BaseType = ::pyo3::PyAny;
    type ThreadChecker = ::pyo3::impl_::pyclass::SendablePyClass<PyDecimal>;
    type PyClassMutability = <<::pyo3::PyAny as ::pyo3::impl_::pyclass::PyClassBaseType>::PyClassMutability as ::pyo3::impl_::pycell::PyClassMutability>::MutableChild;
    type Dict = ::pyo3::impl_::pyclass::PyClassDummySlot;
    type WeakRef = ::pyo3::impl_::pyclass::PyClassDummySlot;
    type BaseNativeType = ::pyo3::PyAny;
    fn items_iter() -> ::pyo3::impl_::pyclass::PyClassItemsIter {
        use ::pyo3::impl_::pyclass::*;
        let collector = PyClassImplCollector::<Self>::new();
        static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
            methods: &[],
            slots: &[],
        };
        PyClassItemsIter::new(&INTRINSIC_ITEMS, collector.py_methods())
    }
    fn doc(py: ::pyo3::Python<'_>) -> ::pyo3::PyResult<&'static ::std::ffi::CStr> {
        use ::pyo3::impl_::pyclass::*;
        static DOC: ::pyo3::sync::GILOnceCell<::std::borrow::Cow<'static, ::std::ffi::CStr>> =
            ::pyo3::sync::GILOnceCell::new();
        DOC.get_or_try_init(py, || {
            let collector = PyClassImplCollector::<Self>::new();
            build_pyclass_doc(
                <PyDecimal as ::pyo3::PyTypeInfo>::NAME,
                c"",
                collector.new_text_signature(),
            )
        })
        .map(::std::ops::Deref::deref)
    }
    fn lazy_type_object() -> &'static ::pyo3::impl_::pyclass::LazyTypeObject<Self> {
        use ::pyo3::impl_::pyclass::LazyTypeObject;
        static TYPE_OBJECT: LazyTypeObject<PyDecimal> = LazyTypeObject::new();
        &TYPE_OBJECT
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
impl PyDecimal {}
impl PyDecimal {
    #[doc(hidden)]
    pub const _PYO3_DEF: ::pyo3::impl_::pymodule::AddClassToModule<Self> =
        ::pyo3::impl_::pymodule::AddClassToModule::new();
}
#[doc(hidden)]
#[allow(non_snake_case)]
impl PyDecimal {}
#[automatically_derived]
impl ::core::fmt::Debug for PyDecimal {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "PyDecimal", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PyDecimal {
    #[inline]
    fn clone(&self) -> PyDecimal {
        let _: ::core::clone::AssertParamIsClone<Decimal>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for PyDecimal {}

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
