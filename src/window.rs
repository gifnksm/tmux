use crate::{ffi, options::Options};
use std::{
    ffi::c_void,
    os::raw::{c_char, c_int},
};

pub(crate) use crate::ffi::{
    window as Window, window_pane as WindowPane, winlink as Winlink, winlinks as Winlinks,
};

impl Winlinks {
    pub(crate) fn prev_mut(&mut self, wl: &mut Winlink) -> Option<&mut Winlink> {
        unsafe { ffi::glue_winlinks_prev(self, wl).as_mut() }
    }

    pub(crate) fn next_mut(&mut self, wl: &mut Winlink) -> Option<&mut Winlink> {
        unsafe { ffi::glue_winlinks_next(self, wl).as_mut() }
    }
}

impl Winlink {
    pub(crate) fn idx(&self) -> i32 {
        self.idx
    }

    pub(crate) fn window_mut(&mut self) -> &'static mut Window {
        unsafe { self.window.as_mut().unwrap() }
    }

    pub(crate) fn set_window(&mut self, w: &mut Window) {
        self.window = w as _;
    }
}

impl Window {
    pub(crate) fn remove_winlink(&mut self, wl: &mut Winlink) {
        unsafe { ffi::glue_window_remove_winlink(self, wl) }
    }

    pub(crate) fn insert_winlink(&mut self, wl: &mut Winlink) {
        unsafe { ffi::glue_window_insert_winlink(self, wl) }
    }

    pub(crate) fn set_name(&mut self, name: *const c_char) {
        unsafe { ffi::window_set_name(self, name) }
    }

    pub(crate) fn options_mut(&mut self) -> &mut Options {
        unsafe { self.options.as_mut().unwrap() }
    }

    pub(crate) fn remove_pane(&mut self, pane: &mut WindowPane) {
        unsafe { ffi::window_remove_pane(self, pane) }
    }

    pub(crate) fn each_panes(&mut self, f: impl FnMut(&mut Self, &mut WindowPane) -> bool) {
        // FIXME: workaround to satisfy borrow checker
        let mut ctx = EachPanesCtx {
            window: self,
            f: Box::new(f) as _,
        };
        unsafe {
            ffi::glue_window_panes_foreach_safe(
                &mut self.panes,
                &mut ctx as *mut _ as _,
                Some(each_panes),
            );
        }
    }
}

struct EachPanesCtx<'a> {
    window: *mut Window,
    f: Box<dyn FnMut(&mut Window, &mut WindowPane) -> bool + 'a>,
}

extern "C" fn each_panes(pane: *mut WindowPane, ctx: *mut c_void) -> c_int {
    let pane = unsafe { pane.as_mut().unwrap() };
    let ctx = unsafe { (ctx as *mut EachPanesCtx).as_mut().unwrap() };
    let window = unsafe { ctx.window.as_mut().unwrap() };
    (ctx.f)(window, pane) as _
}
