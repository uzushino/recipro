use libc::{ c_void, c_int, c_char };
use crate::{ ReciproVM };

use std::ffi::CStr;


type Callback = extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int;
type Closure<'a> = dyn FnMut(*mut c_char, c_int) -> c_int + 'a;

mod ffi {
  use super::*;

  #[link(name = "binding", kind = "static")]
  extern "C" {
    pub fn module_compile(vm: *mut ReciproVM, filename: *const c_char, script: *const c_char) -> c_int;
    pub fn module_instantiate(vm: *mut ReciproVM, id: c_int, data: *mut c_void, f: Callback);
    pub fn module_evaluate(vm: *mut ReciproVM, id: c_int) -> *const c_int;
  }
}

pub struct Module {}

impl Module {
  pub fn compile(vm: *mut ReciproVM, filename: *const c_char, script: *const c_char) -> c_int {
    unsafe { ffi::module_compile(vm, filename, script) }
  }

  pub fn instantiate(vm: *mut ReciproVM, id: i32, closure: &mut Closure) {
    let mut closure_ptr = Box::new(closure);
    let _ = unsafe { 
      let ptr: *mut Closure = &mut *closure_ptr;
      ffi::module_instantiate(vm, id, ptr as *mut c_void, Self::resolve_callback);
    };
  }

  pub fn evaluate(vm: *mut ReciproVM, id: i32) {
    let _ = unsafe { ffi::module_evaluate(vm, id) };
  }

  extern "C" fn resolve_callback(data: *mut c_void, specifier: *mut c_char, id: c_int) -> c_int {
    let mut closure: Box<Box<Closure>> = unsafe { Box::from_raw(data as *mut _) };
    (*closure)(specifier, id)
  }
}