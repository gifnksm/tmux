use crate::{cmd::QueueItem, ffi};
use std::os::raw::c_char;

pub(crate) fn single_from_target(item: &QueueItem, fmt: *const c_char) -> *mut c_char {
    unsafe { ffi::format_single_from_target(item as *const _ as *mut _, fmt) }
}
