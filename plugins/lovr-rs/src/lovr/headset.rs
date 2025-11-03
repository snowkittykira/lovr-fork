use crate::{lovr_sys, lovr, lovr::lovr_assert};

pub fn is_active() -> bool {
    unsafe{
        lovr_sys::lovrHeadsetIsActive()
    }
}

pub fn update() -> lovr::Result<f64> {
    unsafe{
        let mut dt = 0.;
        lovr_assert(lovr_sys::lovrHeadsetUpdate(&mut dt))?;
        Ok(dt)
    }
}

pub fn poll_events() -> lovr::Result<()> {
    unsafe {
        lovr_assert(lovr_sys::lovrHeadsetPollEvents())
    }
}
