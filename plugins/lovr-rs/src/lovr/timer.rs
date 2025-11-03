use crate::{lovr_sys, lovr::*};

pub fn step() -> f64 {
    unsafe {
        lovr_sys::lovrTimerStep()
    }
}
