extern crate libc;

use std::borrow::Cow;
use std::os::raw::c_char;
use std::ffi::CStr;

#[link(name = "binding", kind = "static")]
extern "C" {
    fn get_v8_version() -> *const c_char;
}

pub fn v8_version() -> Cow<'static, str> {
    unsafe { 
        let version = get_v8_version() as *mut _ ;
        CStr::from_ptr(version).to_string_lossy()
    }
}