use crate::{ffi, layout::LayoutCell, options::Options};
use std::{
    ffi::c_void,
    os::raw::{c_char, c_int},
    ptr,
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
    pub(crate) fn active_mut(&mut self) -> Option<&'static mut WindowPane> {
        unsafe { self.active.as_mut() }
    }

    pub(crate) fn last_mut(&mut self) -> Option<&'static mut WindowPane> {
        unsafe { self.last.as_mut() }
    }

    pub(crate) fn set_last(&mut self, last: Option<&mut WindowPane>) {
        self.last = last.map(|p| p as *mut _).unwrap_or_else(ptr::null_mut);
    }

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

    pub(crate) fn push_zoom(&mut self, flag: bool) -> bool {
        unsafe { ffi::window_push_zoom(self, flag as _) != 0 }
    }

    pub(crate) fn pop_zoom(&mut self) -> bool {
        unsafe { ffi::window_pop_zoom(self) != 0 }
    }

    pub(crate) fn remove_pane(&mut self, pane: &mut WindowPane) {
        unsafe { ffi::window_remove_pane(self, pane) }
    }

    pub(crate) fn first_pane(&mut self) -> Option<&'static mut WindowPane> {
        unsafe { ffi::glue_window_first_pane(self).as_mut() }
    }

    pub(crate) fn last_pane(&mut self) -> Option<&'static mut WindowPane> {
        unsafe { ffi::glue_window_last_pane(self).as_mut() }
    }

    pub(crate) fn replace_pane(&mut self, src: &mut WindowPane, dst: &mut WindowPane) {
        unsafe { ffi::glue_window_replace_pane(self, src, dst) }
    }

    pub(crate) fn insert_pane_head(&mut self, wp: &mut WindowPane) {
        unsafe { ffi::glue_window_insert_pane_head(self, wp) }
    }

    pub(crate) fn insert_pane_after(&mut self, list_wp: &mut WindowPane, wp: &mut WindowPane) {
        unsafe { ffi::glue_window_insert_pane_after(self, list_wp, wp) }
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

    pub(crate) fn set_active_pane(&mut self, wp: &mut WindowPane, notify: bool) -> bool {
        unsafe { ffi::window_set_active_pane(self, wp, notify as _) != 0 }
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

impl WindowPane {
    pub(crate) fn set_window(&mut self, w: &mut Window) {
        self.window = w;
    }

    pub(crate) fn options_mut(&mut self) -> &'static mut Options {
        unsafe { self.options.as_mut().unwrap() }
    }

    pub(crate) fn layout_cell_mut(&mut self) -> &'static mut LayoutCell {
        unsafe { self.layout_cell.as_mut().unwrap() }
    }

    pub(crate) fn set_layout_cell(&mut self, lc: &mut LayoutCell) {
        self.layout_cell = lc;
    }

    pub(crate) fn sx(&self) -> u32 {
        self.sx
    }

    pub(crate) fn sy(&self) -> u32 {
        self.sy
    }

    pub(crate) fn xoff(&self) -> u32 {
        self.xoff
    }

    pub(crate) fn set_xoff(&mut self, xoff: u32) {
        self.xoff = xoff;
    }

    pub(crate) fn yoff(&self) -> u32 {
        self.yoff
    }

    pub(crate) fn set_yoff(&mut self, yoff: u32) {
        self.yoff = yoff;
    }

    pub(crate) fn prev_mut(&mut self) -> Option<&'static mut Self> {
        unsafe { ffi::glue_window_pane_prev(self).as_mut() }
    }

    pub(crate) fn next_mut(&mut self) -> Option<&'static mut Self> {
        unsafe { ffi::glue_window_pane_next(self).as_mut() }
    }

    pub(crate) fn resize(&mut self, sx: u32, sy: u32) {
        unsafe { ffi::window_pane_resize(self, sx, sy) }
    }
}
