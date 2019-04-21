extern crate libc;
extern crate failure;

use std::borrow::Cow;
use std::ffi::{ CStr, CString };
use std::os::raw::c_char;

pub enum ReciproVM {}

#[link(name = "binding", kind = "static")]
extern "C" {
    fn v8_init() ;
    fn v8_dispose() ;
    fn v8_shutdown_platform() ;
    fn v8_get_version() -> *const c_char;

    pub fn init() -> *mut ReciproVM;
    pub fn dispose(vm: *mut ReciproVM);
    pub fn execute(vm: *mut ReciproVM, script: *const c_char);
}

pub fn v8_version() -> Cow<'static, str> {
    unsafe { 
        let version = v8_get_version() as *mut _ ;
        CStr::from_ptr(version).to_string_lossy()
    }
}

pub fn initialize() {
    unsafe { v8_init(); }
}

pub fn shutdown() {
    unsafe { 
        v8_dispose(); 
        v8_shutdown_platform();
    }
}

pub fn execute_script(js: String) -> Result<(), failure::Error> {
    let script = CString::new(js.as_str())?;
    
    unsafe { 
        let vm = init();
        execute(vm, script.as_ptr());
        dispose(vm);
    }

    Ok(())
}