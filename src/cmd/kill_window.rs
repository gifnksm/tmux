use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, server};
use cstr::cstr;
use std::{os::raw::c_char, ptr};

/// Destroy window.
#[no_mangle]
static cmd_kill_window_entry: Entry = Entry {
    name: cstr!("kill-window").as_ptr(),
    alias: cstr!("killw").as_ptr(),

    args: Args {
        template: cstr!("at:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-a] [-t target-window]").as_ptr(), // usage: cstr!(concat!("[-a] ", CMD_TARGET_WINDOW_USAGE)).as_ptr(),

    source: EntryFlag::EMPTY,
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
    let target = item.target();
    let wl = target.wl_mut();
    let s = target.s_mut();

    if args.has(b'a') {
        if s.windows_mut().prev_mut(wl).is_none() && s.windows_mut().next_mut(wl).is_none() {
            return Retval::Normal;
        }

        // Kill all windows except the current one.
        loop {
            let mut found = false;
            s.each_windows(|_s, loop_wl| {
                if !ptr::eq(loop_wl.window_mut(), wl.window_mut()) {
                    server::kill_window(loop_wl.window_mut(), false);
                    found = true;
                }
                found
            });
            if !found {
                break;
            }
        }

        // If the current window appears in the session more than once,
        // kill it as well.
        let mut found = 0;
        s.each_windows(|_s, loop_wl| {
            if ptr::eq(loop_wl.window_mut(), wl.window_mut()) {
                found += 1;
            }
            false // continue
        });
        if found > 1 {
            server::kill_window(wl.window_mut(), false);
        }

        server::renumber_all();
        return Retval::Normal;
    }

    server::kill_window(wl.window_mut(), true);
    Retval::Normal
}
