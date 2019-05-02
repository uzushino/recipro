use std::cell::RefCell;
use std::ops::Deref;

use crate::{Engine, Isolate, ReciproVM, Snapshot};

#[repr(C)]
pub struct SnapshotData {
    pub data_ptr: *const u8,
    pub data_size: usize,
}

impl SnapshotData {
    pub fn as_slice<'a>(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.data_ptr, self.data_size) }
    }
}

#[link(name = "binding", kind = "static")]
extern "C" {
    fn init_recipro_core(snapshot: SnapshotData) -> *mut ReciproVM;
    fn init_recipro_snapshot() -> *mut ReciproVM;

    fn dispose(vm: *mut ReciproVM);
    fn take_snapshot(vm: *mut ReciproVM) -> SnapshotData;
    fn delete_snapshot(ptr: *const u8);
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
                data_size: self.snapshot_data.len(),
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

    pub fn delete_snapshot(data_ptr: *const u8) {
        unsafe {
            delete_snapshot(data_ptr);
        }
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
