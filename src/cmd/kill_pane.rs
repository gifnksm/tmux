use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, layout, server};
use cstr::cstr;
use std::{os::raw::c_char, ptr};

/// Kill pane.
#[no_mangle]
static cmd_kill_pane_entry: Entry = Entry {
    name: cstr!("kill-pane").as_ptr(),
    alias: cstr!("killp").as_ptr(),

    args: Args {
        template: cstr!("at:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-a] [-t target-pane]").as_ptr(), // usage: cstr!(concat!("[-a] ", CMD_TARGET_PANE_USAGE)).as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag {
        flag: b't' as c_char,
        type_: FindType::CMD_FIND_PANE,
        flags: 0,
    },

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let args = this.args();
    let target = item.target();
    let winlink = target.wl_mut();
    let wp = target.wp_mut();
    let window = winlink.window_mut();

    if args.has(b'a') != 0 {
        server::unzoom_window(window);
        window.each_panes(|window, pane| {
            if ptr::eq(pane as _, wp as _) {
                return false; // continue
            }
            server::client_remove_pane(pane);
            layout::close_pane(pane);
            window.remove_pane(pane);
            false
        });
        server::redraw_window(window);
        return Retval::Normal;
    }

    server::kill_pane(wp);
    Retval::Normal
}
