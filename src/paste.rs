use crate::ffi;
use std::ptr;

pub(crate) use ffi::paste_buffer as Buffer;

pub(crate) fn walk(pb: Option<&mut Buffer>) -> Option<&mut Buffer> {
    let pb = pb.map(|p| p as *mut Buffer).unwrap_or(ptr::null_mut());
    unsafe { ffi::paste_walk(pb).as_mut() }
}
