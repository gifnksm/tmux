use crate::{
    client::Client,
    cmd::QueueItem,
    ffi,
    paste::Buffer as PasteBuffer,
    session::Session,
    window::{WindowPane, Winlink},
};
use std::{
    os::raw::{c_char, c_int},
    ptr,
};

pub(crate) use ffi::format_tree as Tree;

pub(crate) fn true_(s: *const c_char) -> bool {
    unsafe { ffi::format_true(s) != 0 }
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

pub(crate) fn add(ft: &mut Tree, key: *const c_char, fmt: *const c_char) {
    unsafe { ffi::format_add(ft, key, fmt) }
}

pub(crate) fn expand(ft: &mut Tree, fmt: *const c_char) -> *mut c_char {
    unsafe { ffi::format_expand(ft, fmt) }
}

pub(crate) fn single_from_target(item: &QueueItem, fmt: *const c_char) -> *mut c_char {
    unsafe { ffi::format_single_from_target(item as *const _ as *mut _, fmt) }
}

pub(crate) fn defaults(
    ft: &mut Tree,
    c: Option<&mut Client>,
    s: Option<&mut Session>,
    wl: Option<&mut Winlink>,
    wp: Option<&mut WindowPane>,
) {
    let c = c.map(|c| c as *mut _).unwrap_or(ptr::null_mut());
    let s = s.map(|s| s as *mut _).unwrap_or(ptr::null_mut());
    let wl = wl.map(|wl| wl as *mut _).unwrap_or(ptr::null_mut());
    let wp = wp.map(|wp| wp as *mut _).unwrap_or(ptr::null_mut());
    unsafe { ffi::format_defaults(ft, c, s, wl, wp) }
}

pub(crate) fn defaults_paste_buffer(ft: &mut Tree, pb: &mut PasteBuffer) {
    unsafe { ffi::format_defaults_paste_buffer(ft, pb) }
}
