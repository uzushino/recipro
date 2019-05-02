use std::borrow::Cow;
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::os::raw::c_char;

use crate::{Engine, Platform};

#[link(name = "binding", kind = "static")]
extern "C" {
    fn v8_init();
    fn v8_dispose();
    fn v8_shutdown_platform();
    fn v8_get_version() -> *const c_char;
}

impl<'a> Platform<'a> {
    pub fn version() -> Cow<'static, str> {
        unsafe {
            let version = v8_get_version() as *mut _;
            CStr::from_ptr(version).to_string_lossy()
        }
    }

    pub fn new(engine: &'a Engine) -> Platform {
        unsafe { v8_init() }

        Platform {
            isolate: ManuallyDrop::new(engine),
        }
    }

    pub fn engine_start(&self) {
        self.isolate.init();
    }

    pub fn isolate(&self) -> &'a Engine {
        *self.isolate.deref()
    }

    pub fn shutdown() {
        unsafe {
            v8_dispose();
            v8_shutdown_platform();
        }
    }
}

impl<'a> Drop for Platform<'a> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.isolate);
        }
        //Self::shutdown();
    }
}
