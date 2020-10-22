use crate::{ffi, options::Options};
use std::os::raw::c_char;

pub(crate) use crate::ffi::{window as Window, winlink as Winlink};

impl Winlink {
    pub(crate) fn window_mut(&mut self) -> &mut Window {
        unsafe { self.window.as_mut().unwrap() }
    }
}

impl Window {
    pub(crate) fn set_name(&mut self, name: *const c_char) {
        unsafe { ffi::window_set_name(self, name) }
    }

    pub(crate) fn options_mut(&mut self) -> &mut Options {
        unsafe { self.options.as_mut().unwrap() }
    }
}
