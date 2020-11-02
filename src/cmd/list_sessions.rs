use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::{ffi, format, session::Session};
use cstr::cstr;
use std::ffi::{CStr, CString};

/// List all clients.
#[no_mangle]
static cmd_list_sessions_entry: Entry = Entry {
    name: cstr!("list-sessions").as_ptr(),
    alias: cstr!("ls").as_ptr(),

    args: Args {
        template: cstr!("F:f:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-F format] [-f filter]").as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag::EMPTY,

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

const LIST_SESSIONS_TEMPLATE: &CStr = cstr!(
    "\
    #{session_name}: #{session_windows} windows \
    (created #{t:session_created}) \
    #{?session_grouped, (group ,} \
    #{session_group}#{?session_grouped,),} \
    #{?session_attached, (attached),}\
"
);

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let args = this.args();

    let template = args.get(b'F').unwrap_or(LIST_SESSIONS_TEMPLATE);
    let filter = args.get(b'f');

    let mut n = 0;
    Session::each_sessions(|s| {
        let ft = format::create(item.client(), item, ffi::FORMAT_NONE as i32, 0);
        let fmt = CString::new(format!("{}", n)).unwrap();
        format::add(ft, cstr!("line"), &fmt);
        format::defaults(ft, None, Some(s), None, None);

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

    Retval::Normal
}
