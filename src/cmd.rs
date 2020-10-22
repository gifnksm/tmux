use crate::ffi::{
    cmd as Cmd, cmd_entry as Entry, cmd_entry__bindgen_ty_1 as Args, cmd_entry_flag as EntryFlag,
    cmdq_item as Item,
};
use cstr::cstr;

mod kill_server;
mod start_server;

unsafe impl Sync for Entry {}

impl Args {
    const EMPTY: Self = Self {
        template: cstr!("").as_ptr(),
        lower: 0,
        upper: 0,
    };
}

impl EntryFlag {
    const EMPTY: Self = Self {
        flag: 0,
        type_: 0,
        flags: 0,
    };
}
