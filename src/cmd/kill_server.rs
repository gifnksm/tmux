use super::{Args, Cmd, Entry, EntryFlag, QueueItem};
use crate::ffi;
use cstr::cstr;
use std::ptr;

/// Kill the server and do nothing else.
#[no_mangle]
static cmd_kill_server_entry: Entry = Entry {
    name: cstr!("kill-server").as_ptr(),
    alias: ptr::null(),

    args: Args::EMPTY,
    usage: cstr!("").as_ptr(),

    flags: ffi::CMD_STARTSERVER as i32,
    exec: Some(exec),

    source: EntryFlag::EMPTY,
    target: EntryFlag::EMPTY,
};

extern "C" fn exec(_this: *mut Cmd, _item: *mut QueueItem) -> ffi::cmd_retval {
    unsafe {
        let _ = libc::kill(libc::getpid(), libc::SIGTERM);
    }

    ffi::cmd_retval_CMD_RETURN_NORMAL
}
