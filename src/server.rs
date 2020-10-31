use std::os::raw::c_int;

use crate::{
    ffi,
    session::Session,
    window::{Window, WindowPane, Winlink},
};

pub(crate) fn redraw_session(session: &mut Session) {
    unsafe { ffi::server_redraw_session(session) }
}

pub(crate) fn client_remove_pane(pane: &mut WindowPane) {
    unsafe { ffi::server_client_remove_pane(pane) }
}

pub(crate) fn status_session(s: &mut Session) {
    unsafe { ffi::server_status_session(s) }
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

pub(crate) fn kill_window(w: &mut Window, renumber: bool) {
    unsafe { ffi::server_kill_window(w, renumber as c_int) }
}

pub(crate) fn renumber_all() {
    unsafe { ffi::server_renumber_all() }
}

pub(crate) fn destroy_session(session: &mut Session) {
    unsafe { ffi::server_destroy_session(session) }
}

pub(crate) fn unlink_window(session: &mut Session, wl: &mut Winlink) {
    unsafe { ffi::server_unlink_window(session, wl) }
}

pub(crate) fn unzoom_window(window: &mut Window) {
    unsafe { ffi::server_unzoom_window(window) }
}
