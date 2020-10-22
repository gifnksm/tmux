use crate::{ffi, window::Window};

pub(crate) fn redraw_window_borders(window: &mut Window) {
    unsafe { ffi::server_redraw_window_borders(window) }
}

pub(crate) fn status_window(window: &mut Window) {
    unsafe { ffi::server_status_window(window) }
}
