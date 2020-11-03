use crate::{ffi, key_string::KeyCode};
use std::{ffi::CStr, ptr};

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
