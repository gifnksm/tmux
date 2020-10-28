use crate::{
    arguments,
    ffi::{
        self, cmd as Cmd, cmd_entry as Entry, cmd_entry__bindgen_ty_1 as Args,
        cmd_entry_flag as EntryFlag,
    },
};
use cstr::cstr;

pub(crate) use self::{find::State as FindState, queue::Item as QueueItem};

mod find;
mod queue;

mod kill_pane;
mod kill_server;
mod kill_session;
mod rename_window;
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

impl Cmd {
    pub(crate) fn args(&self) -> &arguments::Args {
        unsafe {
            ffi::cmd_get_args(self as *const _ as *mut _)
                .as_ref()
                .unwrap()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Retval {
    //Error,
    Normal,
    //Wait,
    //Stop,
}

fn exec_wrap(
    this: *mut Cmd,
    item: *mut QueueItem,
    exec: impl FnOnce(&mut Cmd, &mut QueueItem) -> Retval,
) -> ffi::cmd_retval {
    let this = unsafe { this.as_mut().unwrap() };
    let item = unsafe { item.as_mut().unwrap() };

    match exec(this, item) {
        //Retval::Error => ffi::cmd_retval_CMD_RETURN_ERROR,
        Retval::Normal => ffi::cmd_retval_CMD_RETURN_NORMAL,
        //Retval::Wait => ffi::cmd_retval_CMD_RETURN_WAIT,
        //Retval::Stop => ffi::cmd_retval_CMD_RETURN_STOP,
    }
}
