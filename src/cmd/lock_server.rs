use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::{ffi, resize, server};
use cstr::cstr;

/// Lock a server.
#[no_mangle]
static cmd_lock_server_entry: Entry = Entry {
    name: cstr!("lock-server").as_ptr(),
    alias: cstr!("lock").as_ptr(),

    args: Args::EMPTY,
    usage: cstr!("").as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag::EMPTY,

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(_this: &mut Cmd, _item: &mut QueueItem) -> Retval {
    server::lock();
    resize::recalculate_sizes();
    Retval::Normal
}
