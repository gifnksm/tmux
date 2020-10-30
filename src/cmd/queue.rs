use std::ffi::CString;

use super::FindState;
use crate::ffi;

pub(crate) use crate::ffi::cmdq_item as Item;

impl Item {
    pub(crate) fn target(&self) -> &FindState {
        unsafe {
            ffi::cmdq_get_target(self as *const _ as _)
                .as_ref()
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
