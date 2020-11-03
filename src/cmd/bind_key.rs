use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::{cmd_parse, cmd_parse::Status as ParseStatus, ffi, key_bindings, key_string};
use cstr::cstr;
use std::ptr;

/// Bind a key to a command.
#[no_mangle]
static cmd_bind_key_entry: Entry = Entry {
    name: cstr!("bind-key").as_ptr(),
    alias: cstr!("bind").as_ptr(),

    args: Args {
        template: cstr!("nrN:T:").as_ptr(),
        lower: 1,
        upper: -1,
    },
    usage: cstr!(
        "[-nr] [-T key-table] [-N note] key \
        [command [arguments]]"
    )
    .as_ptr(),

    source: EntryFlag::EMPTY,
    target: EntryFlag::EMPTY,

    flags: ffi::CMD_AFTERHOOK as i32,
    exec: Some(exec_c),
};

extern "C" fn exec_c(this: *mut Cmd, item: *mut QueueItem) -> ffi::cmd_retval {
    super::exec_wrap(this, item, exec)
}

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let args = this.args();

    let key = key_string::lookup_string(&args[0]);
    if let ffi::KEYC_NONE | ffi::KEYC_UNKNOWN = key {
        item.error(format!("unknown key: {}", args[0].to_str().unwrap()));
        return Retval::Error;
    }

    let table_name = if let Some(name) = args.get(b'T') {
        name
    } else if args.has(b'n') != 0 {
        cstr!("root")
    } else {
        cstr!("prefix")
    };

    let repeat = args.has(b'r') != 0;
    let note = args.get(b'N');

    if args.argv().len() != 1 {
        let pr = if args.argv().len() == 2 {
            cmd_parse::from_string(&args[1], None)
        } else {
            cmd_parse::from_arguments(&args.argv()[1..], None)
        };

        match pr.status() {
            ParseStatus::Empty => {
                item.error("empty command");
                return Retval::Error;
            }
            ParseStatus::Error => {
                item.error(pr.error().to_str().unwrap());
                pr.free_error();
                return Retval::Error;
            }
            ParseStatus::Success => {}
        }
        key_bindings::add(table_name, key, note, repeat, pr.cmd_list());
    } else {
        key_bindings::add(table_name, key, note, repeat, ptr::null_mut());
    }
    Retval::Normal
}
