use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{client::Client, ffi, format};
use cstr::cstr;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr,
};

/// List all clients.
#[no_mangle]
static cmd_list_clients_entry: Entry = Entry {
    name: cstr!("list-clients").as_ptr(),
    alias: cstr!("lsc").as_ptr(),

    args: Args {
        template: cstr!("F:f:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-F format] [-t target-session]").as_ptr(), // usage: cstr!(concat!("[-F format] ", CMD_TARGET_SESSION_USAGE)).as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag {
        flag: b't' as c_char,
        type_: FindType::CMD_FIND_SESSION,
        flags: 0,
    },

    flags: (ffi::CMD_READONLY | ffi::CMD_AFTERHOOK) as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

const LIST_CLIENTS_TEMPLATE: &CStr = cstr!(
    "\
    #{client_name}: #{session_name} \
    [#{client_width}x#{client_height} #{client_termname}] \
    #{?client_flags,(,}#{client_flags}#{?client_flags,),}\
"
);

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let args = this.args();
    let target = item.target();

    let s = if args.has(b't') != 0 {
        Some(target.s_mut())
    } else {
        None
    };

    let template = args.get(b'F').unwrap_or(LIST_CLIENTS_TEMPLATE);

    let mut idx = 0;
    Client::each_clients(|c| {
        match (c.session(), s.as_ref()) {
            (None, _) => return false,
            (Some(cs), Some(s)) if ptr::eq(&*cs, &**s) => return false,
            _ => {}
        }

        let ft = format::create(item.client(), item, ffi::FORMAT_NONE as i32, 0);
        let fmt = CString::new(format!("{}", idx)).unwrap();
        format::add(ft, cstr!("line"), &fmt);
        format::defaults(ft, Some(c), None, None, None);

        let line = format::expand(ft, &template);
        item.print(line.to_str().unwrap());

        format::free(ft);

        idx += 1;

        false
    });

    Retval::Normal
}
