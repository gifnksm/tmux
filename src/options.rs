use crate::ffi;
use std::os::raw::{c_char, c_longlong};

pub(crate) use crate::ffi::{options as Options, options_entry as Entry};

impl Options {
    pub(crate) fn set_number(&mut self, name: *const c_char, value: c_longlong) -> &mut Entry {
        unsafe { ffi::options_set_number(self, name, value).as_mut().unwrap() }
    }
}
