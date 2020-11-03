use crate::{client::Client, cmd_find::State as FindState, ffi};
use cstr::cstr;
use std::ffi::CString;

pub(crate) use crate::ffi::cmdq_item as Item;

impl Item {
    pub(crate) fn target(&self) -> &'static FindState {
        unsafe {
            ffi::cmdq_get_target(self as *const _ as _)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn client(&self) -> &'static mut Client {
        unsafe {
            ffi::cmdq_get_client(self as *const _ as _)
                .as_mut()
                .unwrap()
        }
    }

    pub(crate) fn target_client(&self) -> &'static mut Client {
        unsafe {
            ffi::cmdq_get_target_client(self as *const _ as _)
                .as_mut()
                .unwrap()
        }
    }

    pub(crate) fn print(&mut self, msg: impl Into<Vec<u8>>) {
        let msg = CString::new(msg).unwrap();
        unsafe {
            ffi::cmdq_print(self, cstr!("%s").as_ptr(), msg.as_ptr());
        }
    }

    pub(crate) fn error(&mut self, msg: impl Into<Vec<u8>>) {
        let msg = CString::new(msg).unwrap();
        unsafe {
            ffi::cmdq_error(self, cstr!("%s").as_ptr(), msg.as_ptr());
        }
    }
}
