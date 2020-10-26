use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::{ffi, format, server};
use cstr::cstr;
use std::ffi::c_void;

/// Rename a window.
#[no_mangle]
static cmd_rename_window_entry: Entry = Entry {
    name: cstr!("rename-window").as_ptr(),
    alias: cstr!("renamew").as_ptr(),

    args: Args {
        template: cstr!("t:").as_ptr(),
        lower: 1,
        upper: 1,
    },
    usage: cstr!("[-t target-window] new-name").as_ptr(), // usage: cstr!(concat!(CMD_TARGET_WINDOW_USAGE, " new-name")).as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag::EMPTY,

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let argv = this.args().argv();
    let new_name = format::single_from_target(item, argv[0]);

    let target = item.target();
    let wl = target.wl_mut();
    let window = wl.window_mut();
    window.set_name(new_name);
    window
        .options_mut()
        .set_number(cstr!("automatic-rename").as_ptr(), 0);

    server::redraw_window_borders(window);
    server::status_window(window);

    unsafe { libc::free(new_name as *mut c_void) };

    Retval::Normal
}
