use std::rc::Rc;
use std::mem::ManuallyDrop;
use std::borrow::Cow;
use std::ffi::CStr;

use crate::{Engine, Platform, Isolate, Snapshot};

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

impl<'a> Platform {
    pub fn version() -> Cow<'static, str> {
        unsafe {
            let version = ffi::v8_get_version() as *mut _;
            CStr::from_ptr(version).to_string_lossy()
        }
    }

    pub fn new() -> Platform {
        Self::init();

        Platform {
            engines: ManuallyDrop::new(Vec::new()),
        }
    }

    pub fn add_engine(&mut self, engine: Rc<Engine>)  {
        engine.init();
        self.engines.push(engine);
    }

    pub fn init() {
        println!("init");
        unsafe { ffi::v8_init() }
    }

    pub fn shutdown() {
        unsafe {
            ffi::v8_dispose();
            ffi::v8_shutdown_platform();
        }
    }
}

impl Drop for Platform {
    fn drop(&mut self)  {
        unsafe {
            println!("ManyallyDrop");
            ManuallyDrop::drop(&mut self.engines)
        }

        println!("Shotdown");
        Self::shutdown();
    }
}