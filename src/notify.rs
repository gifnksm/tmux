use crate::{ffi, session::Session};
use std::os::raw::c_char;

pub(crate) fn session(name: *const c_char, s: &mut Session) {
    unsafe { ffi::notify_session(name, s) }
}
