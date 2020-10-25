use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::ffi;
use cstr::cstr;
use std::ptr;

/// Starts the server and do nothing
#[no_mangle]
static cmd_start_server_entry: Entry = Entry {
    name: cstr!("start-server").as_ptr(),
    alias: ptr::null(),

    args: Args::EMPTY,
    usage: cstr!("").as_ptr(),

    flags: ffi::CMD_STARTSERVER as i32,
    exec: Some(exec_c),

    source: EntryFlag::EMPTY,
    target: EntryFlag::EMPTY,
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(_this: &mut Cmd, _item: &mut QueueItem) -> Retval {
    Retval::Normal
}
