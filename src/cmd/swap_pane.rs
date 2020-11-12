use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, server};
use cstr::cstr;
use std::{os::raw::c_char, ptr};

/// Swap two panes.
#[no_mangle]
static cmd_swap_pane_entry: Entry = Entry {
    name: cstr!("swap-pane").as_ptr(),
    alias: cstr!("swapp").as_ptr(),

    args: Args {
        template: cstr!("dDs:t:UZ").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-dDUZ] [-s src-pane] [-t dst-pane]").as_ptr(), // usage: cstr!(concat!("[-dDUZ] ", CMD_SRCDST_PANE_USAGE)).as_ptr(),

    source: EntryFlag {
        flag: b's' as c_char,
        type_: FindType::CMD_FIND_PANE,
        flags: ffi::CMD_FIND_DEFAULT_MARKED as i32,
    },
    target: EntryFlag {
        flag: b't' as c_char,
        type_: FindType::CMD_FIND_PANE,
        flags: 0,
    },

    flags: 0,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let args = this.args();
    let source = item.source();
    let target = item.target();

    let dst_w = target.wl_mut().window_mut();
    let dst_wp = target.wp_mut();

    if dst_w.push_zoom(args.has(b'Z')) {
        server::redraw_window(dst_w);
    }

    let src_w;
    let src_wp;
    if args.has(b'D') {
        src_w = target.wl_mut().window_mut();
        src_wp = dst_wp.next_mut().or_else(|| dst_w.first_pane()).unwrap();
    } else if args.has(b'U') {
        src_w = target.wl_mut().window_mut();
        src_wp = dst_wp.prev_mut().or_else(|| dst_w.last_pane()).unwrap();
    } else {
        src_w = source.wl_mut().window_mut();
        src_wp = source.wp_mut();
    }

    if !ptr::eq(src_w, dst_w) && src_w.push_zoom(args.has(b'Z')) {
        server::redraw_window(src_w);
    }

    if !ptr::eq(src_wp, dst_wp) {
        server::client_remove_pane(src_wp);
        server::client_remove_pane(dst_wp);

        let mut tmp_wp = dst_wp.prev_mut();
        dst_w.remove_pane(dst_wp);
        src_w.replace_pane(src_wp, dst_wp);
        if tmp_wp.is_some() && ptr::eq(*tmp_wp.as_ref().unwrap(), src_wp) {
            tmp_wp = Some(dst_wp);
        }
        if let Some(tmp_wp) = tmp_wp {
            dst_w.insert_pane_after(tmp_wp, src_wp);
        } else {
            dst_w.insert_pane_head(src_wp);
        }

        let src_lc = src_wp.layout_cell_mut();
        let dst_lc = dst_wp.layout_cell_mut();
        src_lc.set_wp(dst_wp);
        dst_wp.set_layout_cell(src_lc);
        dst_lc.set_wp(src_wp);
        src_wp.set_layout_cell(dst_lc);

        src_wp.set_window(dst_w);
        src_wp.options_mut().set_parent(dst_w.options_mut());
        src_wp.flags |= ffi::PANE_STYLECHANGED as i32;
        dst_wp.set_window(src_w);
        dst_wp.options_mut().set_parent(src_w.options_mut());
        dst_wp.flags |= ffi::PANE_STYLECHANGED as i32;

        let sx = src_wp.sx();
        let sy = src_wp.sy();
        let xoff = src_wp.xoff();
        let yoff = src_wp.yoff();
        src_wp.set_xoff(dst_wp.xoff());
        src_wp.set_yoff(dst_wp.yoff());
        src_wp.resize(dst_wp.sx(), dst_wp.sy());
        dst_wp.set_xoff(xoff);
        dst_wp.set_yoff(yoff);
        dst_wp.resize(sx, sy);

        if args.has(b'd') {
            if !ptr::eq(src_w, dst_w) {
                src_w.set_active_pane(dst_wp, true);
                dst_w.set_active_pane(src_wp, true);
            } else {
                src_w.set_active_pane(dst_wp, true);
            }
        } else {
            if src_w.active_mut().is_some() && ptr::eq(src_w.active_mut().unwrap(), src_wp) {
                src_w.set_active_pane(dst_wp, true);
            }
            if dst_w.active_mut().is_some() && ptr::eq(dst_w.active_mut().unwrap(), dst_wp) {
                dst_w.set_active_pane(src_wp, true);
            }
        }
        if !ptr::eq(src_w, dst_w) {
            if src_w.last_mut().is_some() && ptr::eq(src_w.last_mut().unwrap(), src_wp) {
                src_w.set_last(None);
            }
            if dst_w.last_mut().is_some() && ptr::eq(dst_w.last_mut().unwrap(), dst_wp) {
                dst_w.set_last(None);
            }
        }
        server::redraw_window(src_w);
        server::redraw_window(dst_w);
    }

    if src_w.pop_zoom() {
        server::redraw_window(src_w);
    }
    if !ptr::eq(src_w, dst_w) && dst_w.pop_zoom() {
        server::redraw_window(dst_w);
    }

    Retval::Normal
}
