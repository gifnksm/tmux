use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, format, session::Session};
use cstr::cstr;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

/// List windows on given session.
#[no_mangle]
static cmd_list_windows_entry: Entry = Entry {
    name: cstr!("list-windows").as_ptr(),
    alias: cstr!("lss").as_ptr(),

    args: Args {
        template: cstr!("F:f:at:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-a] [-F format] [-f filter] [-t target-session]").as_ptr(), // usage: cstr!(concat!("[-a] [-F format] [-f filter] ", CMD_TARGET_SESSION_USAGE)).as_ptr(),

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

const LIST_WINDOWS_TEMPLATE: &CStr = cstr!(
    "\
    #{window_index}: #{window_name}#{window_flags} \
    (#{window_panes} panes) \
    [#{window_width}x#{window_height}] \
    [layout #{window_layout}] #{window_id}\
    #{?window_active, (active),}\
    "
);

const LIST_WINDOWS_WITH_SESSION_TEMPLATE: &CStr = cstr!(
    "\
    #{session_name}:\
    #{window_index}: #{window_name}#{window_flags} \
    (#{window_panes} panes) \
    [#{window_width}x#{window_height}] \
    "
);

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let args = this.args();
    let target = item.target();

    if args.has(b'a') {
        server(this, item)
    } else {
        session(this, target.s_mut(), item, Type::Session);
    }

    Retval::Normal
}

enum Type {
    All,
    Session,
}

fn server(this: &mut Cmd, item: &mut QueueItem) {
    Session::each_sessions(|s| {
        session(this, s, item, Type::All);
        false
    });
}

fn session(this: &mut Cmd, s: &mut Session, item: &mut QueueItem, ty: Type) {
    let args = this.args();
    let template = args.get(b'F').unwrap_or_else(|| match ty {
        Type::All => LIST_WINDOWS_WITH_SESSION_TEMPLATE,
        Type::Session => LIST_WINDOWS_TEMPLATE,
    });
    let filter = args.get(b'f');

    let mut n = 0;
    s.each_windows(|s, wl| {
        let ft = format::create(item.client(), item, ffi::FORMAT_NONE as i32, 0);
        let fmt = CString::new(format!("{}", n)).unwrap();
        format::add(ft, cstr!("line"), &fmt);
        format::defaults(ft, None, Some(s), Some(wl), None);

        let flag;
        if let Some(filter) = filter {
            let expanded = format::expand(ft, &filter);
            flag = format::true_(&expanded);
        } else {
            flag = true;
        }

        if flag {
            let line = format::expand(ft, &template);
            item.print(line.to_str().unwrap());
        }

        format::free(ft);
        n += 1;

        false
    });
}
