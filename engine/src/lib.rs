extern crate libc;

use std::borrow::Cow;
use std::os::raw::c_char;
use std::ffi::CStr;

#[link(name = "binding", kind = "static")]
extern "C" {
    fn v8_init() ;
    fn v8_dispose() ;
    fn v8_shutdown_platform() ;

    fn get_v8_version() -> *const c_char;

    fn js_eval(script: *const i8);
}

pub fn init() {
    unsafe { v8_init() };
}

pub fn dispose() {
    unsafe { v8_dispose() };
}

pub fn shutdown_platform() {
    unsafe{ v8_shutdown_platform() };
}

pub fn v8_version() -> Cow<'static, str> {
    unsafe { 
        let version = get_v8_version() as *mut _ ;
        CStr::from_ptr(version).to_string_lossy()
    }
}

pub fn eval() {
    let script = std::ffi::CString::new("'Hello from rust!'").unwrap();

    unsafe { 
        js_eval(script.as_ptr() as *const i8); 
    }
}