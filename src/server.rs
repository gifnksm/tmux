use crate::{
    ffi,
    window::{Window, WindowPane},
};

pub(crate) fn client_remove_pane(pane: &mut WindowPane) {
    unsafe { ffi::server_client_remove_pane(pane) }
}

pub(crate) fn redraw_window(window: &mut Window) {
    unsafe { ffi::server_redraw_window(window) }
}

pub(crate) fn redraw_window_borders(window: &mut Window) {
    unsafe { ffi::server_redraw_window_borders(window) }
}

pub(crate) fn status_window(window: &mut Window) {
    unsafe { ffi::server_status_window(window) }
}

pub(crate) fn kill_pane(pane: &mut WindowPane) {
    unsafe { ffi::server_kill_pane(pane) }
}

pub(crate) fn unzoom_window(window: &mut Window) {
    unsafe { ffi::server_unzoom_window(window) }
}
