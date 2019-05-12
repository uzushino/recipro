use std::os::raw::{ c_int, c_char, c_void };

use crate::{ ReciproVM };

type Callback = extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int;
type Closure = dyn FnMut(*const c_char, c_int) -> c_int;

mod ffi {
  use super::*;

  #[link(name = "binding", kind = "static")]
  extern "C" {
    pub fn module_compile(vm: *mut ReciproVM, filename: *const c_char, script: *const c_char) -> c_int;
    pub fn module_instantiate(vm: *mut ReciproVM, data: *mut c_void, id: c_int, f: Callback);
    pub fn module_evaluate(vm: *mut ReciproVM, id: c_int) -> *const c_int;
  }
}

pub struct Module {}

impl Module {
  pub fn compile(vm: *mut ReciproVM, filename: *const c_char, script: *const c_char) -> c_int {
    unsafe { ffi::module_compile(vm, filename, script) }
  }

  pub fn instantiate(vm: *mut ReciproVM, id: i32, closure: &mut Closure) {
    let _ = unsafe { 
      let closure_ptr: &mut Closure = closure;
      ffi::module_instantiate(vm, closure_ptr as *mut _ as *mut c_void, id, Self::resolve_callback) 
    };
  }

  pub fn evaluate(vm: *mut ReciproVM, id: i32) {
    let _ = unsafe { ffi::module_evaluate(vm, id) };
  }

  extern "C" fn resolve_callback(data: *mut c_void, specifier: *const c_char, id: c_int) -> c_int {
    let closure: &mut &mut Closure = unsafe { std::mem::transmute(data) };
    closure(specifier, id)
  }
}