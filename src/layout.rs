use crate::{ffi, window::WindowPane};

pub(crate) fn close_pane(pane: &mut WindowPane) {
    unsafe { ffi::layout_close_pane(pane) }
}
