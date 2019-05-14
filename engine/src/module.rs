use std::ffi::CString;

use libc::{ c_void, c_int, c_char };

use crate::{ ReciproVM };

type ModId = i32;
type Callback = extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int;
type Closure<'a> = dyn FnMut(*mut c_char, c_int) -> c_int + 'a;

mod ffi {
  use super::*;

  #[link(name = "binding", kind = "static")]
  extern "C" {
    pub fn module_compile(vm: *mut ReciproVM, filename: *const u8, script: *const c_char) -> ModId;
    pub fn module_instantiate(vm: *mut ReciproVM, id: ModId, data: *mut c_void, f: Callback);
    pub fn module_evaluate(vm: *mut ReciproVM, id: ModId) -> *const ModId;
  }
}

pub struct Module {
  id: std::cell::Cell<ModId>,
  vm: *mut ReciproVM,
}

impl Module {
  pub fn new(vm: *mut ReciproVM) -> Self {
    Module {
      id: std::cell::Cell::new(0),
      vm,
    }
  }

  pub fn module_id(&self) -> ModId {
    self.id.get()
  }

  pub fn compile_from_script(&self, filename: &str) -> Result<(), failure::Error> {
    let source = std::fs::read_to_string(filename)?;
    let script = CString::new(source)?;
    let id = unsafe { ffi::module_compile(self.vm, filename.as_ptr(), script.as_ptr()) };
    self.id.set(id);

    Ok(())
  }

  pub fn compile(&self, filename: &str, script: &str) -> Result<(), failure::Error> {
    let script = CString::new(script.as_bytes())?;
    let id = unsafe { ffi::module_compile(self.vm, filename.as_ptr(), script.as_ptr()) };
    self.id.set(id);

    Ok(())
  }

  pub fn instantiate(&self, closure: &mut Closure) {
    let mut closure_ptr = Box::new(closure);

    unsafe { 
      ffi::module_instantiate(
        self.vm, 
        self.id.get(), 
        &mut *closure_ptr as *mut _  as *mut c_void, 
        Self::resolve_callback
      );
    };
  }

  pub fn evaluate(&self) {
    unsafe { ffi::module_evaluate(self.vm, self.id.get()) };
  }

  extern "C" fn resolve_callback(data: *mut c_void, specifier: *mut c_char, id: c_int) -> c_int {
    unsafe {
      let closure: &mut &mut Closure =  &mut *(data as *mut _);
      closure(specifier, id)
    }
  }
}