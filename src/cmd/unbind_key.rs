use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::{ffi, key_bindings, key_string};
use cstr::cstr;

/// Unbind key from command.
#[no_mangle]
static cmd_unbind_key_entry: Entry = Entry {
    name: cstr!("unbind-key").as_ptr(),
    alias: cstr!("unbind").as_ptr(),

    args: Args {
        template: cstr!("anqT:").as_ptr(),
        lower: 0,
        upper: 1,
    },
    usage: cstr!("[-anq] [-T key-table] key").as_ptr(),

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
    let quiet = args.has(b'q');

    if args.has(b'a') {
        if !args.argv().is_empty() {
            if !quiet {
                item.error("key given with -a");
            }
            return Retval::Error;
        }

        let table_name = args.get(b'T').unwrap_or_else(|| {
            if args.has(b'n') {
                cstr!("root")
            } else {
                cstr!("prefix")
            }
        });

        if key_bindings::get_table(table_name, false).is_none() {
            if !quiet {
                item.error(format!(
                    "table {} doesn't exist",
                    table_name.to_str().unwrap()
                ));
            }
            return Retval::Error;
        }
        key_bindings::remove_table(table_name);
        return Retval::Normal;
    }

    if args.argv().len() != 1 {
        if !quiet {
            item.error("missing key");
        }
        return Retval::Error;
    }

    let key = key_string::lookup_string(&args[0]);
    if let ffi::KEYC_NONE | ffi::KEYC_UNKNOWN = key {
        if !quiet {
            item.error(format!("unknown key: {}", args[0].to_str().unwrap()));
        }
        return Retval::Error;
    }

    let table_name;
    if let Some(tn) = args.get(b'T') {
        table_name = tn;
        if key_bindings::get_table(table_name, false).is_none() {
            if !quiet {
                item.error(format!(
                    "table {} doesn't exist",
                    table_name.to_str().unwrap()
                ));
            }
            return Retval::Error;
        }
    } else if args.has(b'n') {
        table_name = cstr!("root");
    } else {
        table_name = cstr!("prefix");
    };
    key_bindings::remove(table_name, key);
    Retval::Normal
}
