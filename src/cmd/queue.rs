use std::ffi::CString;

use super::FindState;
use crate::{client::Client, ffi};

pub(crate) use crate::ffi::cmdq_item as Item;

impl Item {
    pub(crate) fn target(&self) -> &FindState {
        unsafe {
            ffi::cmdq_get_target(self as *const _ as _)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn target_client(&self) -> &mut Client {
        unsafe {
            ffi::cmdq_get_target_client(self as *const _ as _)
                .as_mut()
                .unwrap()
        }
    }

    pub(crate) fn error(&mut self, msg: impl Into<Vec<u8>>) {
        let msg = CString::new(msg).unwrap();
        unsafe {
            ffi::cmdq_error(self, msg.as_ptr());
        }
    }
}
