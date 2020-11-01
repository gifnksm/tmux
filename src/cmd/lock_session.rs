use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::{ffi, resize, server};
use cstr::cstr;
use std::os::raw::c_char;

/// Lock a session.
#[no_mangle]
static cmd_lock_session_entry: Entry = Entry {
    name: cstr!("lock-session").as_ptr(),
    alias: cstr!("locks").as_ptr(),

    args: Args {
        template: cstr!("t:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-t target-session]").as_ptr(), // usage: cstr!(CMD_TARGET_SESSION_USAGE).as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag {
        flag: b't' as c_char,
        type_: ffi::cmd_find_type_CMD_FIND_SESSION,
        flags: 0,
    },

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(_this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let target = item.target();
    server::lock_session(target.s_mut());
    resize::recalculate_sizes();
    Retval::Normal
}
