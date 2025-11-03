use std::backtrace::Backtrace;
use std::ffi::CStr;

use std::result::Result as StdResult;

use crate::lovr_sys;

mod callbacks;
pub use callbacks::{Callbacks, RunCommand};

pub mod event;
pub mod graphics;
pub mod headset;
pub mod system;
pub mod timer;

mod entry;

pub type Result<T> = StdResult<T, LovrError>;

#[derive(Debug)]
pub struct LovrError(String, Backtrace);

impl From<LovrError> for mlua::Error {
    fn from(err: LovrError) -> Self {
        let bt = err.1.to_string().lines()
            .take_while(|line| !line.contains(": lovr_rs::lovr_run::{{closure}}"))
            .collect::<Vec<_>>()
            .join("\n");
        mlua::Error::RuntimeError(err.0 + "\n\n" + &bt + "\n")
    }
}

fn lovr_assert(condition: bool) -> Result<()> {
    if condition {
        return Ok(())
    }
    unsafe {
        let ptr = lovr_sys::lovrGetError();
        let msg = if ptr.is_null() {
            "Unknown LOVR error".to_string()
        } else {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        };
        Err(LovrError(msg, Backtrace::force_capture()))
    }
}
