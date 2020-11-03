use super::{Args, Cmd, Entry, EntryFlag, FindType, QueueItem, Retval};
use crate::{ffi, format, session::Session, window::Winlink};
use cstr::cstr;
use std::{ffi::CString, os::raw::c_char};

/// List panes on given window.
#[no_mangle]
static cmd_list_panes_entry: Entry = Entry {
    name: cstr!("list-panes").as_ptr(),
    alias: cstr!("lsp").as_ptr(),

    args: Args {
        template: cstr!("asF:f:t:").as_ptr(),
        lower: 0,
        upper: 0,
    },
    usage: cstr!("[-as] [-F format] [-f filter] [-t target-window]").as_ptr(), // usage: cstr!(concat!("[-a] [-F format] [-f filter] ", CMD_TARGET_WINDOW_USAGE)).as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag {
        flag: b't' as c_char,
        type_: FindType::CMD_FIND_WINDOW,
        flags: 0,
    },

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let args = this.args();
    let target = item.target();
    let s = target.s_mut();
    let wl = target.wl_mut();

    if args.has(b'a') != 0 {
        server(this, item)
    } else if args.has(b's') != 0 {
        session(this, s, item, Type::Window);
    } else {
        window(this, s, wl, item, Type::Pane)
    }

    Retval::Normal
}

#[derive(Copy, Clone)]
enum Type {
    Pane,
    Window,
    Session,
}

fn server(this: &mut Cmd, item: &mut QueueItem) {
    Session::each_sessions(|s| {
        session(this, s, item, Type::Session);
        false
    })
}

fn session(this: &mut Cmd, s: &mut Session, item: &mut QueueItem, ty: Type) {
    s.each_windows(|s, wl| {
        window(this, s, wl, item, ty);
        false
    })
}

fn window(this: &mut Cmd, s: &mut Session, wl: &mut Winlink, item: &mut QueueItem, ty: Type) {
    let args = this.args();

    let template = args.get(b'F').unwrap_or_else(|| match ty {
        Type::Pane => cstr!(
            "\
                #{pane_index}: \
                [#{pane_width}x#{pane_height}] [history \
                #{history_size}/#{history_limit}, \
                #{history_bytes} bytes] #{pane_id}\
                #{?pane_active, (active),}#{?pane_dead, (dead),}\
            "
        ),
        Type::Window => cstr!(
            "\
                #{window_index}.#{pane_index}: \
                [#{pane_width}x#{pane_height}] [history \
                #{history_size}/#{history_limit}, \
                #{history_bytes} bytes] #{pane_id}\
                #{?pane_active, (active),}#{?pane_dead, (dead),}\
            "
        ),
        Type::Session => cstr!(
            "\
                #{session_name}:#{window_index}.\
                #{pane_index}: [#{pane_width}x#{pane_height}] \
                [history #{history_size}/#{history_limit}, \
                #{history_bytes} bytes] #{pane_id}\
                #{?pane_active, (active),}#{?pane_dead, (dead),}\
            "
        ),
    });
    let filter = args.get(b'f');

    let mut n = 0;
    wl.window_mut().each_panes(|_w, wp| {
        let ft = format::create(item.client(), item, ffi::FORMAT_NONE as i32, 0);
        let fmt = CString::new(format!("{}", n)).unwrap();
        format::add(ft, cstr!("line"), &fmt);
        format::defaults(ft, None, Some(s), Some(wl), Some(wp));

        let flag;
        if let Some(filter) = filter {
            let expanded = format::expand(ft, filter);
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
    })
}
