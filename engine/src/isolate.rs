use std::ffi::CString;
use std::os::raw::{ c_char, c_void };

pub enum ReciproVM {}

#[repr(C)]
pub struct Snapshot {
  pub data_ptr: *const u8,
  pub data_size: usize,
}

#[link(name = "binding", kind = "static")]
extern "C" {
    fn init(snapshot: *mut Snapshot) -> *mut ReciproVM;
    fn init_snapshot() -> *mut ReciproVM;

    fn dispose(vm: *mut ReciproVM);
    fn execute(vm: *mut ReciproVM, script: *const c_char);

    fn take_snapshot(vm: *mut ReciproVM) -> *mut Snapshot;
    pub fn delete_snapshot(vm: *mut Snapshot) ;
}

pub struct Isolate {
  isolate: *mut ReciproVM,
}

impl Isolate {
  pub fn new(snapshot: *mut Snapshot) -> Isolate {
    let vm = unsafe { init(snapshot) };

    Isolate {
      isolate: vm,
    }
  }
  
  pub fn new_snapshot() -> Isolate {
    let vm = unsafe { init_snapshot() };

    Isolate {
      isolate: vm,
    }
  }

  pub fn execute(&self, js: String) -> Result<(), failure::Error> {
      let script = CString::new(js.as_str())?;
      
      unsafe { 
        execute(self.isolate, script.as_ptr()); 
      }

      Ok(())
  }

  pub fn snapshot(&self) -> *mut Snapshot {
      unsafe { 
        take_snapshot(self.isolate)
      }
  }
}

impl Drop for Isolate {
  fn drop(&mut self) {
    unsafe { 
      dispose(self.isolate); 
    }
  }
}
