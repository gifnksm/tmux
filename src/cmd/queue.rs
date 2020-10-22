use super::FindState;
use crate::ffi;

pub(crate) use crate::ffi::cmdq_item as Item;

impl Item {
    pub(crate) fn target_mut(&mut self) -> &mut FindState {
        unsafe { ffi::cmdq_get_target(self).as_mut().unwrap() }
    }
}
