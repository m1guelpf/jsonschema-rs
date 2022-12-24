use crate::string;
use pyo3::ffi::{
    PyDict_New, PyFloat_FromDouble, PyImport_ImportModule, PyList_New, PyLong_FromLongLong,
    PyMapping_GetItemString, PyObject, PyObject_GetAttrString, PyTuple_New, PyTypeObject,
    PyUnicode_FromString, Py_DECREF, Py_None, Py_TYPE, Py_True,
};
use std::{os::raw::c_char, sync::Once};

pub static mut TRUE: *mut pyo3::ffi::PyObject = 0 as *mut pyo3::ffi::PyObject;

pub static mut STR_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut INT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut BOOL_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut NONE_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut FLOAT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut LIST_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut DICT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut TUPLE_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut ENUM_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut VALUE_STR: *mut PyObject = 0 as *mut PyObject;

static INIT: Once = Once::new();

// Taken from orjson
#[cold]
unsafe fn look_up_enum_type() -> *mut PyTypeObject {
    let module = PyImport_ImportModule("enum\0".as_ptr().cast::<c_char>());
    let module_dict = PyObject_GetAttrString(module, "__dict__\0".as_ptr().cast::<c_char>());
    let ptr = PyMapping_GetItemString(module_dict, "EnumMeta\0".as_ptr().cast::<c_char>())
        .cast::<PyTypeObject>();
    Py_DECREF(module_dict);
    Py_DECREF(module);
    ptr
}

pub(crate) unsafe fn get_type_name(object_type: *mut pyo3::ffi::PyTypeObject) -> String {
    let type_name =
        PyObject_GetAttrString(object_type as _, "__name__\0".as_ptr().cast::<c_char>());
    let mut str_size: pyo3::ffi::Py_ssize_t = 0;
    let uni = string::read_utf8_from_str(type_name, &mut str_size);
    let bytes = std::slice::from_raw_parts(uni, str_size as usize).to_vec();
    String::from_utf8_unchecked(bytes)
}

/// Set empty type object pointers with their actual values.
/// We need these Python-side type objects for direct comparison during conversion to serde types
/// NOTE. This function should be called before any serialization logic
pub fn init() {
    INIT.call_once(|| unsafe {
        TRUE = Py_True();
        STR_TYPE = Py_TYPE(PyUnicode_FromString("".as_ptr().cast::<c_char>()));
        DICT_TYPE = Py_TYPE(PyDict_New());
        TUPLE_TYPE = Py_TYPE(PyTuple_New(0_isize));
        LIST_TYPE = Py_TYPE(PyList_New(0_isize));
        NONE_TYPE = Py_TYPE(Py_None());
        BOOL_TYPE = Py_TYPE(TRUE);
        INT_TYPE = Py_TYPE(PyLong_FromLongLong(0));
        FLOAT_TYPE = Py_TYPE(PyFloat_FromDouble(0.0));
        ENUM_TYPE = look_up_enum_type();
        VALUE_STR = pyo3::ffi::PyUnicode_InternFromString("value\0".as_ptr().cast::<c_char>());
    });
}
