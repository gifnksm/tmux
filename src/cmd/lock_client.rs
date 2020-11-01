use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::{ffi, resize, server};
use cstr::cstr;

/// Lock a client.
#[no_mangle]
static cmd_lock_client_entry: Entry = Entry {
    name: cstr!("lock-client").as_ptr(),
    alias: cstr!("lockc").as_ptr(),

    args: Args {
        template: cstr!("t:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-t target-client]").as_ptr(), // usage: cstr!(CMD_TARGET_CLIENT_USAGE).as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag::EMPTY,

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(_this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let tc = item.target_client();
    server::lock_client(tc);
    resize::recalculate_sizes();
    Retval::Normal
}
