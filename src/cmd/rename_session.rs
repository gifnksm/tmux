use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, format, notify, server, session::Session};
use cstr::cstr;
use std::os::raw::c_char;

/// Rename a window.
#[no_mangle]
static cmd_rename_session_entry: Entry = Entry {
    name: cstr!("rename-session").as_ptr(),
    alias: cstr!("rename").as_ptr(),

    args: Args {
        template: cstr!("t:").as_ptr(),
        lower: 1,
        upper: 1,
    },
    usage: cstr!("[-t target-session] new-name").as_ptr(), // usage: cstr!(concat!(CMD_TARGET_SESSION_USAGE, " new-name")).as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag {
        flag: b't' as c_char,
        type_: FindType::CMD_FIND_SESSION,
        flags: 0,
    },

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let target = item.target();
    let s = target.s_mut();

    let tmp = format::single_from_target(item, &this.args()[0]);
    let new_name = Session::check_name(&tmp);

    if &*tmp == s.name() {
        return Retval::Normal;
    }

    if Session::find(&new_name).is_some() {
        item.error(format!("duplicate session: {}", new_name.to_str().unwrap()));
        return Retval::Error;
    }

    Session::remove(s);
    s.set_name(new_name);
    Session::insert(s);

    server::status_session(s);
    notify::session(cstr!("session-renamed").as_ptr(), s);

    Retval::Normal
}
