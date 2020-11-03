use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, resize, server};
use cstr::cstr;
use std::os::raw::c_char;

/// Unlink window.
#[no_mangle]
static cmd_unlink_window_entry: Entry = Entry {
    name: cstr!("unlink-window").as_ptr(),
    alias: cstr!("unlinkw").as_ptr(),

    args: Args {
        template: cstr!("kt:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-k] [-t target-window]").as_ptr(), // usage: cstr!(concat!("[-k] ", CMD_TARGET_WINDOW_USAGE)).as_ptr(),

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
    let w = wl.window_mut();
    let s = target.s_mut();

    if args.has(b'k') && s.is_linked(w) {
        item.error("window only linked to one session");
        return Retval::Error;
    }

    server::unlink_window(s, wl);
    resize::recalculate_sizes();

    Retval::Normal
}
