use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_int,
};

use crate::{
    ffi,
    window::{Window, Winlink, Winlinks},
};

pub(crate) use {ffi::session as Session, ffi::session_group as SessionGroup};

impl Session {
    pub(crate) fn find(name: &CStr) -> Option<&'static mut Session> {
        unsafe { ffi::session_find(name.as_ptr()).as_mut() }
    }

    pub(crate) fn check_name(name: &CStr) -> CString {
        let ptr = unsafe { ffi::session_check_name(name.as_ptr()) };
        unsafe { ffi::ptr_into_cstring(ptr) }
    }

    pub(crate) fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name) }
    }

    pub(crate) fn set_name(&mut self, name: CString) {
        unsafe { libc::free(self.name as *mut c_void) };
        self.name = unsafe { libc::strdup(name.as_ptr()) };
    }

    pub(crate) fn windows_mut(&mut self) -> &mut Winlinks {
        &mut self.windows
    }

    pub(crate) fn remove(s: *mut Self) -> *mut Self {
        unsafe { ffi::glue_sessions_remove(s) }
    }

    pub(crate) fn insert(s: *mut Self) -> *mut Self {
        unsafe { ffi::glue_sessions_insert(s) }
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

    pub(crate) fn select(&self, idx: i32) -> i32 {
        unsafe { ffi::session_select(self as *const _ as *mut _, idx as _) as _ }
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

impl SessionGroup {
    pub(crate) fn contains(s: &Session) -> Option<&'static mut SessionGroup> {
        unsafe { ffi::session_group_contains(s as *const _ as _).as_mut() }
    }

    pub(crate) fn synchronize_from(s: &Session) {
        unsafe { ffi::session_group_synchronize_from(s as *const _ as _) }
    }
}
