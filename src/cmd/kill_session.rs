use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, server, session::Session};
use cstr::cstr;
use std::{os::raw::c_char, ptr};

/// Destroy session, detaching all clients attached to it and destroying any
/// windows linked only to this session.
///
/// Note this deliberately has no alias to make it hard to hit by accident.
#[no_mangle]
static cmd_kill_session_entry: Entry = Entry {
    name: cstr!("kill-session").as_ptr(),
    alias: ptr::null(),

    args: Args {
        template: cstr!("aCt:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-aC] [-t target-session]").as_ptr(), // usage: cstr!(concat!("[-a] ", CMD_TARGET_SESSION_USAGE)).as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag {
        flag: b't' as c_char,
        type_: FindType::CMD_FIND_SESSION,
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

    if args.has(b'C') != 0 {
        let s = target.s_mut();
        s.each_windows(|_s, wl| {
            wl.window_mut().flags &= !(ffi::WINDOW_ALERTFLAGS as i32);
            wl.flags &= !(ffi::WINLINK_ALERTFLAGS as i32);
            false // continue
        });
        server::redraw_session(s);
    } else if args.has(b'a') != 0 {
        Session::each_sessions(|s| {
            if !ptr::eq(s, target.s_mut()) {
                server::destroy_session(s);
                s.destroy(true, concat!(file!(), line!()));
            }
            false // continue
        })
    } else {
        let s = target.s_mut();
        server::destroy_session(s);
        s.destroy(true, concat!(file!(), line!()));
    }

    Retval::Normal
}
