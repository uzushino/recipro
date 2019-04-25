use std::ffi::CString;
use std::os::raw::{ c_char, c_void };
use std::cell::RefCell;
use std::ops::Deref;

use crate::{ Engine, Isolate, Snapshot, ReciproVM };

#[repr(C)]
pub struct SnapshotData {
  pub data_ptr: *const u8,
  pub data_size: usize,
}

#[link(name = "binding", kind = "static")]
extern "C" {
    fn init_recipro_core(snapshot: SnapshotData) -> *mut ReciproVM;
    fn init_recipro_snapshot() -> *mut ReciproVM;

    fn dispose(vm: *mut ReciproVM);
    fn eval(vm: *mut ReciproVM, script: *const c_char);

    fn take_snapshot(vm: *mut ReciproVM) -> SnapshotData;
    fn delete_snapshot(vm: *const u8) ;
}

impl<'a> Isolate<'a> {
  pub fn new(snapshot: &[u8]) -> Isolate {
    Isolate {
      snapshot_data: snapshot,
      vm: RefCell::new(std::ptr::null_mut()),
    }
  }
}

impl<'a> Engine for Isolate<'a> { 
  fn core(&self) -> *mut ReciproVM {
    *self.vm.borrow_mut()
  }

  fn init(&self) {
    let vm = unsafe { 
      let snapshot = SnapshotData {
        data_ptr: self.snapshot_data.as_ref().as_ptr(),
        data_size: self.snapshot_data.len()
      };

      init_recipro_core(snapshot)
    };

    self.vm.replace(vm);
  }
}

impl Snapshot {
  pub fn new() -> Snapshot {
    Snapshot {
      vm: RefCell::new(std::ptr::null_mut()),
    }
  }
  
  pub fn snapshot(&self) -> SnapshotData {
    unsafe { take_snapshot(*self.vm.borrow().deref()) }
  }
}

impl Engine for Snapshot { 
  fn core(&self) -> *mut ReciproVM {
    *self.vm.borrow_mut()
  }
  
  fn init(&self) {
    let vm = unsafe { init_recipro_snapshot() };
    self.vm.replace(vm);
  }
}

impl<'a> Drop for Isolate<'a> {
  fn drop(&mut self) {
    unsafe {
      delete_snapshot(self.snapshot_data.as_ref().as_ptr());
      dispose(*self.vm.get_mut()); 
    }
  }
}

impl Drop for Snapshot {
  fn drop(&mut self) {
    unsafe { 
      dispose(*self.vm.get_mut()); 
    }
  }
}
