use std::borrow::Cow;
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::ops::Deref;

use crate::{Engine, Platform};

mod ffi {
    use libc::c_char;

    #[link(name = "binding", kind = "static")]
    extern "C" {
        pub fn v8_init();
        pub fn v8_dispose();
        pub fn v8_shutdown_platform();
        pub fn v8_get_version() -> *const c_char;
    }
}

impl<'a> Platform<'a> {
    pub fn version() -> Cow<'static, str> {
        unsafe {
            let version = ffi::v8_get_version() as *mut _;
            CStr::from_ptr(version).to_string_lossy()
        }
    }

    pub fn new(engine: &'a Engine) -> Platform {
        unsafe { ffi::v8_init() }

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
            ffi::v8_dispose();
            ffi::v8_shutdown_platform();
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
