use std::borrow::Cow;
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::ops::Deref;

use crate::isolate::Isolate;

#[link(name = "binding", kind = "static")]
extern "C" {
    fn v8_init() ;
    fn v8_dispose() ;
    fn v8_shutdown_platform() ;
    fn v8_get_version() -> *const c_char;
}

pub struct Platform {
  isolate: ManuallyDrop<Isolate>,
}

impl Platform {
  pub fn version() -> Cow<'static, str> {
      unsafe { 
          let version = v8_get_version() as *mut _ ;
          CStr::from_ptr(version).to_string_lossy()
      }
  }

  pub fn new() -> Platform {
      unsafe { v8_init(); }

      Platform {
        isolate: ManuallyDrop::new(Isolate::new()),
      }
  }

  pub fn isolate(&self) -> &Isolate {
    self.isolate.deref()
  }

  pub fn shutdown() {
      unsafe { 
          v8_dispose(); 
          v8_shutdown_platform();
      }
  }
}

impl Drop for Platform {
  fn drop(&mut self) {
    unsafe {
      ManuallyDrop::drop(&mut self.isolate);
    }

    Self::shutdown();
  }
}