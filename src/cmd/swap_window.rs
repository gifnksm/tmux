use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, resize, server, session::SessionGroup};
use cstr::cstr;
use std::{os::raw::c_char, ptr};

/// Swap one window with another.
#[no_mangle]
static cmd_swap_window_entry: Entry = Entry {
    name: cstr!("swap-window").as_ptr(),
    alias: cstr!("swapw").as_ptr(),

    args: Args {
        template: cstr!("ds:t:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-d] [-s src-window] [-t dst-window]").as_ptr(), // usage: cstr!(concat!("[-d] ", CMD_SRCDST_WINDOW_USAGE)).as_ptr(),

    source: EntryFlag {
        flag: b's' as c_char,
        type_: FindType::CMD_FIND_WINDOW,
        flags: ffi::CMD_FIND_DEFAULT_MARKED as i32,
    },
    target: EntryFlag {
        flag: b't' as c_char,
        type_: FindType::CMD_FIND_WINDOW,
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
    let src = source.s_mut();
    let dst = target.s_mut();
    let wl_src = source.wl_mut();
    let wl_dst = target.wl_mut();

    let sg_src = SessionGroup::contains(src);
    let sg_dst = SessionGroup::contains(dst);

    if let (Some(sg_src), Some(sg_dst)) = (sg_src, sg_dst) {
        if !ptr::eq(src, dst) && ptr::eq(sg_src, sg_dst) {
            item.error("can't move window, sessions are grouped");
            return Retval::Error;
        }
    }

    if ptr::eq(wl_src.window_mut(), wl_dst.window_mut()) {
        return Retval::Normal;
    }

    let w_dst = wl_dst.window_mut();
    w_dst.remove_winlink(wl_dst);
    let w_src = wl_src.window_mut();
    w_src.remove_winlink(wl_src);

    wl_dst.set_window(w_src);
    w_src.insert_winlink(wl_dst);
    wl_src.set_window(w_dst);
    w_dst.insert_winlink(wl_src);

    if args.has(b'd') {
        dst.select(wl_dst.idx());
        if !ptr::eq(src, dst) {
            src.select(wl_src.idx());
        }
    }

    SessionGroup::synchronize_from(src);
    server::redraw_session_group(src);
    if !ptr::eq(src, dst) {
        SessionGroup::synchronize_from(dst);
        server::redraw_session_group(dst);
    }
    resize::recalculate_sizes();

    Retval::Normal
}
