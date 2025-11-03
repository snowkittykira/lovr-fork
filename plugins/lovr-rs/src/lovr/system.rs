use crate::{lovr_sys, lovr::*};

pub fn poll_events() {
    unsafe {
        lovr_sys::lovrSystemPollEvents()
    }
}
