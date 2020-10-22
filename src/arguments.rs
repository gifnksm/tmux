use crate::ffi;
pub(crate) use ffi::args as Args;
use std::{os::raw::c_char, slice};

impl Args {
    pub(crate) fn argv(&self) -> &[*mut c_char] {
        unsafe { slice::from_raw_parts(self.argv, self.argc as usize) }
    }
}
