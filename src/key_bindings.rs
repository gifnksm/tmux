use crate::{ffi, key_string::KeyCode};
use std::{ffi::CStr, ptr};

pub(crate) use ffi::key_table as KeyTable;

pub(crate) fn get_table(name: &CStr, create: bool) -> Option<&'static mut KeyTable> {
    unsafe { ffi::key_bindings_get_table(name.as_ptr(), create as _).as_mut() }
}

pub(crate) fn add(
    name: &CStr,
    key: KeyCode,
    note: Option<&CStr>,
    repeat: bool,
    cmd_list: *mut ffi::cmd_list,
) {
    let note = note.map(|s| s.as_ptr()).unwrap_or_else(ptr::null);
    unsafe { ffi::key_bindings_add(name.as_ptr(), key, note, repeat as _, cmd_list) }
}

pub(crate) fn remove(name: &CStr, key: KeyCode) {
    unsafe { ffi::key_bindings_remove(name.as_ptr(), key) }
}

pub(crate) fn remove_table(name: &CStr) {
    unsafe { ffi::key_bindings_remove_table(name.as_ptr()) }
}
