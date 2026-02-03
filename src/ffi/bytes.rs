// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2022-2026)

use crate::ffi::{Py_ssize_t, PyObject};
#[cfg(Py_3_15)]
use crate::ffi::{PyVarObject, Py_hash_t};
use core::ffi::c_char;

#[cfg(not(Py_3_15))]
pub(crate) use pyo3_ffi::PyBytesObject;

#[cfg(Py_3_15)]
#[repr(C)]
pub(crate) struct PyBytesObject {
    pub ob_base: PyVarObject,
    pub ob_shash: Py_hash_t,
    pub ob_sval: [c_char; 1],
}

#[cfg(CPython)]
#[allow(non_snake_case)]
#[inline(always)]
pub(crate) unsafe fn PyBytes_AS_STRING(op: *mut PyObject) -> *const c_char {
    unsafe { (&raw const (*op.cast::<crate::ffi::PyBytesObject>()).ob_sval).cast::<c_char>() }
}

#[cfg(not(CPython))]
#[allow(non_snake_case)]
#[inline(always)]
pub(crate) unsafe fn PyBytes_AS_STRING(op: *mut PyObject) -> *const c_char {
    unsafe { pyo3_ffi::PyBytes_AsString(op) }
}

#[allow(non_snake_case)]
#[inline(always)]
pub(crate) unsafe fn PyBytes_GET_SIZE(op: *mut PyObject) -> Py_ssize_t {
    unsafe { super::compat::Py_SIZE(op) }
}
