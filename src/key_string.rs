use crate::ffi;
use std::ffi::CStr;

pub(crate) use ffi::key_code as KeyCode;

pub(crate) fn lookup_string(string: &CStr) -> KeyCode {
    unsafe { ffi::key_string_lookup_string(string.as_ptr()) }
}
