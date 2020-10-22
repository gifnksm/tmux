use crate::{ffi, window::Winlink};
pub(crate) use ffi::cmd_find_state as State;

impl State {
    pub(crate) fn wl_mut(&mut self) -> &mut Winlink {
        unsafe { self.wl.as_mut().unwrap() }
    }
}
