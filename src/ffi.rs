#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::redundant_static_lifetimes)]

use std::{
    ffi::CString,
    ffi::{c_void, CStr},
    os::raw::c_char,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! c_try {
    ($e:expr, $ret:expr) => {
        match $e {
            Ok(s) => s,
            Err(_) => return $ret,
        }
    };
}

pub(crate) unsafe fn ptr_into_cstring(ptr: *mut c_char) -> CString {
    let s = CStr::from_ptr(ptr).to_owned();
    libc::free(ptr as *mut c_void);
    s
}
