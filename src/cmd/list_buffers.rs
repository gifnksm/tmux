use super::{Args, Cmd, Entry, EntryFlag, QueueItem, Retval};
use crate::{ffi, format, paste};
use cstr::cstr;
use std::ffi::CStr;

/// List paste buffers.
#[no_mangle]
static cmd_list_buffers_entry: Entry = Entry {
    name: cstr!("list-buffers").as_ptr(),
    alias: cstr!("lsb").as_ptr(),

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

const LIST_BUFFERS_TEMPLATE: &CStr =
    cstr!(r##"#{buffer_name}: #{buffer_size} bytes: "#{buffer_sample}""##);

fn exec(this: &mut Cmd, item: &mut QueueItem) -> Retval {
    let args = this.args();

    let template = args.get(b'F').unwrap_or(LIST_BUFFERS_TEMPLATE);
    let filter = args.get(b'f');

    let mut pb = None;
    loop {
        pb = paste::walk(pb);
        let pb = match &mut pb {
            Some(pb) => pb,
            None => break,
        };

        let ft = format::create(item.client(), item, ffi::FORMAT_NONE as _, 0);
        format::defaults_paste_buffer(ft, pb);

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
    }

    Retval::Normal
}
