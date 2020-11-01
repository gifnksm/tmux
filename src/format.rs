use crate::{client::Client, cmd::QueueItem, ffi, paste::Buffer as PasteBuffer};
use std::os::raw::{c_char, c_int};

pub(crate) use ffi::format_tree as Tree;

pub(crate) fn true_(s: *const c_char) -> bool {
    unsafe { ffi::format_true(s) !=0 }
}

pub(crate) fn create(
    c: &mut Client,
    item: &QueueItem,
    tag: c_int,
    flags: c_int,
) -> &'static mut Tree {
    unsafe {
        ffi::format_create(c as _, item as *const _ as _, tag, flags)
            .as_mut()
            .unwrap()
    }
}

pub(crate) fn free(ft: &mut Tree) {
    unsafe { ffi::format_free(ft) }
}

pub(crate) fn expand(ft: &mut Tree, fmt: *const c_char) -> *mut c_char {
    unsafe { ffi::format_expand(ft, fmt) }
}

pub(crate) fn single_from_target(item: &QueueItem, fmt: *const c_char) -> *mut c_char {
    unsafe { ffi::format_single_from_target(item as *const _ as *mut _, fmt) }
}

pub(crate) fn defaults_paste_buffer(ft: &mut Tree, pb: &mut PasteBuffer) {
    unsafe { ffi::format_defaults_paste_buffer(ft, pb) }
}
