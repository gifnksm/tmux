use crate::ffi;
use std::{ffi::CStr, os::raw::c_char, ptr};

pub(crate) use ffi::{cmd_parse_input as Input, cmd_parse_result as Result};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Status {
    Empty,
    Error,
    Success,
}

impl From<ffi::cmd_parse_status> for Status {
    fn from(s: ffi::cmd_parse_status) -> Self {
        match s {
            ffi::cmd_parse_status::CMD_PARSE_EMPTY => Status::Empty,
            ffi::cmd_parse_status::CMD_PARSE_ERROR => Status::Error,
            ffi::cmd_parse_status::CMD_PARSE_SUCCESS => Status::Success,
        }
    }
}

impl Result {
    pub(crate) fn status(&self) -> Status {
        self.status.into()
    }
    pub(crate) fn error(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.error) }
    }
    pub(crate) fn free_error(&mut self) {
        unsafe { libc::free(self.error as _) };
        self.error = ptr::null_mut();
    }
    pub(crate) fn cmd_list(&self) -> *mut ffi::cmd_list {
        self.cmdlist
    }
}

pub(crate) fn from_string(s: &CStr, pi: Option<&mut Input>) -> &'static mut Result {
    let pi = pi.map(|ptr| ptr as *mut _).unwrap_or(ptr::null_mut());
    unsafe { ffi::cmd_parse_from_string(s.as_ptr(), pi).as_mut().unwrap() }
}

pub(crate) fn from_arguments(args: &[*mut c_char], pi: Option<&mut Input>) -> &'static mut Result {
    let argc = args.len() as _;
    let argv = args.as_ptr();
    let pi = pi.map(|ptr| ptr as *mut _).unwrap_or(ptr::null_mut());
    unsafe {
        ffi::cmd_parse_from_arguments(argc, argv as _, pi)
            .as_mut()
            .unwrap()
    }
}
