use crate::{ffi, window::WindowPane};

pub(crate) use ffi::layout_cell as LayoutCell;

pub(crate) fn close_pane(pane: &mut WindowPane) {
    unsafe { ffi::layout_close_pane(pane) }
}

impl LayoutCell {
    pub(crate) fn set_wp(&mut self, wp: &mut WindowPane) {
        self.wp = wp as _;
    }
}
