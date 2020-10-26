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
}
