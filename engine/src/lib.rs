use std::sync::Arc;
use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;

pub mod isolate;
pub mod platform;

use std::mem::ManuallyDrop;

#[link(name = "binding", kind = "static")]
extern "C" {
    fn eval(vm: *mut ReciproVM, script: *const c_char);
}

pub enum ReciproVM {}

pub struct Platform {
    pub engines: ManuallyDrop<Vec<Arc<Engine>>>,
}

pub trait Engine : Sync + Send {
    fn new() -> Self where Self: Sized;
    fn core(&self) -> *mut ReciproVM;
    fn init(&self);
    fn execute_script<'a>(&self, js: &'a str) -> Result<(), failure::Error> {
        let script = CString::new(js)?;
        unsafe {
            eval(self.core(), script.as_ptr());
        }
        Ok(())
    }
    fn run_script<'a>(&self, script: &'a str) -> Result<(), failure::Error> {
        let script = std::fs::read_to_string(script)?;
        self.execute_script(script.as_str())?;

        Ok(())
    }
}

pub struct Isolate<'a> {
    snapshot_data: Option<&'a [u8]>,
    vm: RefCell<*mut ReciproVM>,
}

unsafe impl<'a> Send for Isolate<'a> {}
unsafe impl<'a> Sync for Isolate<'a> {}

pub struct Snapshot {
    vm: RefCell<*mut ReciproVM>,
}

unsafe impl Send for Snapshot {}
unsafe impl Sync for Snapshot {}
