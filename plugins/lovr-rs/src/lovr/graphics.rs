use glam::Mat4;

use crate::{lovr_sys, lovr, lovr::lovr_assert};

pub fn get_window_pass() -> lovr::Result<Option<Pass>> {
    unsafe {
        let mut pass_ptr: *mut lovr_sys::Pass = std::ptr::null_mut();
        lovr_assert(lovr_sys::lovrGraphicsGetWindowPass(&mut pass_ptr))?;
        Ok(Pass::from_raw(pass_ptr))
    }
}

pub fn submit(passes: &mut [Pass]) -> lovr::Result<()> {
    unsafe {
        let mut pass_ptrs: Vec<*mut lovr_sys::Pass> = passes.iter_mut().map(|p| p.ptr).collect();
        lovr_assert(lovr_sys::lovrGraphicsSubmit(pass_ptrs.as_mut_ptr(), passes.len().try_into().unwrap()))
    }
}

pub fn present() -> lovr::Result<()> {
    unsafe {
        lovr_assert(lovr_sys::lovrGraphicsPresent())
    }
}

pub struct Pass {
    ptr: *mut lovr_sys::Pass
}

impl Pass {
    pub unsafe fn from_raw(ptr: *mut lovr_sys::Pass) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            unsafe {
                lovr_sys::lovrRetain(ptr as *mut std::ffi::c_void);
            }
            Some(Self { ptr })
        }
    }
}

impl Drop for Pass {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                lovr_sys::lovrRelease(
                    self.ptr as *mut std::ffi::c_void,
                    Some(lovr_sys::lovrPassDestroy));
            }
        }
    }
}

impl Pass {
    pub fn sphere(&mut self, transform: Mat4, longitudes: u32, latitudes:u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassSphere(self.ptr, transform.to_cols_array().as_mut_ptr(), longitudes, latitudes))
        }
    }
}
