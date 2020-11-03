use crate::ffi;
pub(crate) use ffi::args as Args;
use std::{ffi::CStr, ops::Index, os::raw::c_char, slice};

impl Args {
    pub(crate) fn argv(&self) -> &[*mut c_char] {
        unsafe { slice::from_raw_parts(self.argv, self.argc as usize) }
    }

    pub(crate) fn has(&self, flag: u8) -> bool {
        unsafe { ffi::args_has(self as *const _ as _, flag) != 0 }
    }

    pub(crate) fn get(&self, flag: u8) -> Option<&CStr> {
        unsafe {
            ffi::args_get(self as *const _ as _, flag)
                .as_ref()
                .map(|p| CStr::from_ptr(p as _))
        }
    }
}

impl Index<usize> for Args {
    type Output = CStr;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { CStr::from_ptr(self.argv()[index]) }
    }
}
