use crate::ffi;
pub(crate) use ffi::args as Args;
use std::{
    os::raw::{c_char, c_int},
    slice,
};

impl Args {
    pub(crate) fn argv(&self) -> &[*mut c_char] {
        unsafe { slice::from_raw_parts(self.argv, self.argc as usize) }
    }

    pub(crate) fn has(&self, flag: u8) -> c_int {
        unsafe { ffi::args_has(self as *const _ as _, flag) }
    }
}
