use crate::ffi;

pub(crate) fn recalculate_sizes() {
    unsafe { ffi::recalculate_sizes() }
}
