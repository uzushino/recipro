use std::cell::RefCell;
use std::ops::Deref;

use crate::{Engine, Isolate, ReciproVM, Snapshot};

mod ffi {
    use super::*;

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
        pub fn init_recipro_core(snapshot: SnapshotData) -> *mut ReciproVM;
        pub fn init_recipro_snapshot() -> *mut ReciproVM;

        pub fn dispose(vm: *mut ReciproVM);
        pub fn take_snapshot(vm: *mut ReciproVM) -> SnapshotData;
        pub fn delete_snapshot(ptr: *const u8);
    }
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
            let snapshot = ffi::SnapshotData {
                data_ptr: self.snapshot_data.as_ref().as_ptr(),
                data_size: self.snapshot_data.len(),
            };

            ffi::init_recipro_core(snapshot)
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

    pub fn snapshot(&self) -> ffi::SnapshotData {
        unsafe { ffi::take_snapshot(*self.vm.borrow().deref()) }
    }

    pub fn delete_snapshot(data_ptr: *const u8) {
        unsafe {
            ffi::delete_snapshot(data_ptr);
        }
    }
}

impl Engine for Snapshot {
    fn core(&self) -> *mut ReciproVM {
        *self.vm.borrow_mut()
    }

    fn init(&self) {
        let vm = unsafe { ffi::init_recipro_snapshot() };
        self.vm.replace(vm);
    }
}

impl<'a> Drop for Isolate<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::dispose(*self.vm.get_mut());
        }
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        unsafe {
            ffi::dispose(*self.vm.get_mut());
        }
    }
}
