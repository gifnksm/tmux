use crate::{
    ffi,
    session::Session,
    window::{WindowPane, Winlink},
};
pub(crate) use ffi::{cmd_find_state as State, cmd_find_type as Type};

impl State {
    pub(crate) fn wl_mut(&self) -> &mut Winlink {
        unsafe { self.wl.as_mut().unwrap() }
    }

    pub(crate) fn wp_mut(&self) -> &mut WindowPane {
        unsafe { self.wp.as_mut().unwrap() }
    }

    pub(crate) fn s_mut(&self) -> &mut Session {
        unsafe { self.s.as_mut().unwrap() }
    }
}
