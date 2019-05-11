extern crate failure;
extern crate libc;

use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;

pub mod isolate;
pub mod platform;
pub mod module;

use std::mem::ManuallyDrop;

#[link(name = "binding", kind = "static")]
extern "C" {
    fn eval(vm: *mut ReciproVM, script: *const c_char);
}

pub enum ReciproVM {}

pub struct Platform<'a> {
    isolate: ManuallyDrop<&'a Engine>,
}

pub trait Engine {
    fn core(&self) -> *mut ReciproVM;
    fn init(&self);
    fn eval(&self, js: String) -> Result<(), failure::Error> {
        let script = CString::new(js.as_str())?;
        unsafe {
            eval(self.core(), script.as_ptr());
        }
        Ok(())
    }
}

pub struct Isolate<'a> {
    snapshot_data: &'a [u8],
    vm: RefCell<*mut ReciproVM>,
}

pub struct Snapshot {
    vm: RefCell<*mut ReciproVM>,
}
