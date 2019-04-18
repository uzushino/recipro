extern crate libc;

use std::borrow::Cow;
use std::ffi::{ CStr, CString };
use std::os::raw::c_char;

#[link(name = "binding", kind = "static")]
extern "C" {
    fn v8_init() ;
    fn v8_dispose() ;
    fn v8_shutdown_platform() ;

    fn get_v8_version() -> *const c_char;

    fn js_eval(script: *const c_char);
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

pub fn eval(js: String) {
    let script = CString::new(js.as_str())
        .unwrap();

    unsafe { js_eval(script.as_ptr()); }
}