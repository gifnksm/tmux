use std::{
    ffi::{c_void, CString},
    os::raw::c_int,
};

use crate::{
    ffi,
    window::{Window, Winlink, Winlinks},
};

pub(crate) use ffi::session as Session;

impl Session {
    pub(crate) fn windows_mut(&mut self) -> &mut Winlinks {
        &mut self.windows
    }

    pub(crate) fn each_sessions(f: impl FnMut(&mut Session) -> bool) {
        // FIXME: workaround to satisfy borrow checker
        let mut ctx = EachSessionsCtx {
            f: Box::new(f) as _,
        };
        unsafe {
            ffi::glue_sessions_foreach_safe(
                &mut ffi::sessions,
                &mut ctx as *mut _ as _,
                Some(each_sessions),
            );
        }
    }

    pub(crate) fn each_windows(&mut self, f: impl FnMut(&mut Self, &mut Winlink) -> bool) {
        // FIXME: workaround to satisfy borrow checker
        let mut ctx = EachWindowsCtx {
            session: self,
            f: Box::new(f) as _,
        };
        unsafe {
            ffi::glue_winlinks_foreach_safe(
                &mut self.windows,
                &mut ctx as *mut _ as _,
                Some(each_windows),
            );
        }
    }

    pub(crate) fn destroy(&mut self, notify: bool, from: &str) {
        let from = CString::new(from).unwrap();
        unsafe {
            ffi::session_destroy(self, notify as c_int, from.as_ptr());
        }
    }

    pub(crate) fn is_linked(&self, w: &Window) -> bool {
        unsafe { ffi::session_is_linked(self as *const _ as *mut _, w as *const _ as *mut _) != 0 }
    }
}

struct EachSessionsCtx<'a> {
    f: Box<dyn FnMut(&mut Session) -> bool + 'a>,
}

extern "C" fn each_sessions(session: *mut Session, ctx: *mut c_void) -> c_int {
    let session = unsafe { session.as_mut().unwrap() };
    let ctx = unsafe { (ctx as *mut EachSessionsCtx).as_mut().unwrap() };
    (ctx.f)(session) as _
}

struct EachWindowsCtx<'a> {
    session: *mut Session,
    f: Box<dyn FnMut(&mut Session, &mut Winlink) -> bool + 'a>,
}

extern "C" fn each_windows(wl: *mut Winlink, ctx: *mut c_void) -> c_int {
    let wl = unsafe { wl.as_mut().unwrap() };
    let ctx = unsafe { (ctx as *mut EachWindowsCtx).as_mut().unwrap() };
    let session = unsafe { ctx.session.as_mut().unwrap() };
    (ctx.f)(session, wl) as _
}
