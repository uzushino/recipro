use std::ffi::CString;
use std::os::raw::c_char;

pub enum ReciproVM {}

#[link(name = "binding", kind = "static")]
extern "C" {
    fn init() -> *mut ReciproVM;
    fn dispose(vm: *mut ReciproVM);
    fn execute(vm: *mut ReciproVM, script: *const c_char);
}


pub struct Isolate {
  isolate: *mut ReciproVM,
}

impl Isolate {
  pub fn new() -> Isolate {
    let vm = unsafe { init() };

    Isolate {
      isolate: vm,
    }
  }

  pub fn execute(&self, js: String) -> Result<(), failure::Error> {
      let script = CString::new(js.as_str())?;
      unsafe { execute(self.isolate, script.as_ptr()); }
      Ok(())
  }
}

impl Drop for Isolate {
  fn drop(&mut self) {
    unsafe { dispose(self.isolate); }
  }
}
