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

    impl Default for SnapshotData {
        fn default() -> Self {
            SnapshotData {
                data_ptr: std::ptr::null(),
                data_size: 0,
            }
        }
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

impl<'a> Engine for Isolate<'a> {
    fn new() -> Isolate<'a> where Self: Sized {
        Isolate {
            snapshot_data: None,
            vm: RefCell::new(std::ptr::null_mut()),
        }
    }

    fn core(&self) -> *mut ReciproVM {
        *self.vm.borrow_mut()
    }

    fn init(&self) {
        let snapshot = match self.snapshot_data {
            Some(snapshot) => {
                ffi::SnapshotData {
                    data_ptr: snapshot.as_ref().as_ptr(),
                    data_size: snapshot.len(),
                }
            },
            None => ffi::SnapshotData::default()
        };

        let vm = unsafe {
            ffi::init_recipro_core(snapshot)
        };

        self.vm.replace(vm);
    }
}

impl<'a> Isolate<'a> {
    pub fn load_snapshot(&mut self, snapshot: &[u8]) {
        unsafe {
            let b = snapshot.as_ptr();
            let data: &'a [u8] = std::slice::from_raw_parts(b, snapshot.len());
            self.snapshot_data = Some(data);
        }
    }
}

impl Snapshot {
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
    fn new() -> Snapshot where Self: Sized {
        Snapshot {
            vm: RefCell::new(std::ptr::null_mut()),
        }
    }

    fn core(&self) -> *mut ReciproVM {
        *self.vm.borrow_mut()
    }

    fn init(&self) {
        let vm = unsafe { 
            ffi::init_recipro_snapshot() 
        };

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


#[cfg(test)]
mod test {
    use std::fs;
    use std::rc::Rc;

    use crate::Platform;
    use crate::Isolate;

    use super::*;

    use std::sync::{Once, ONCE_INIT};

    static INIT: Once = ONCE_INIT;

    pub fn init_platform() {
        INIT.call_once(|| {
            let platform: Platform = Platform::new();
            std::mem::forget(platform)
        });
    }

    #[test]
    fn evaluate_script() {
        init_platform();

        let engine = Isolate::new();
        engine.init();

        let r = engine.eval("a = 1;");
        assert!(r.is_ok());
    }

    const SNAPSHOT_PATH: &'static str = "/tmp/snapshot";

    fn write_snapshot(script: &str) -> Result<(), failure::Error> {
        let engine = Snapshot::new();
        engine.init();
        engine.eval(script)?;

        let snapshot = engine.snapshot();
        if snapshot.data_size > 0 {
            std::fs::write(SNAPSHOT_PATH, snapshot.as_slice())?;
            Snapshot::delete_snapshot(snapshot.data_ptr);
        }

        Ok(())
    }

    #[test]
    fn snapshot_evaluate() -> Result<(), failure::Error> {
        init_platform();
        
        write_snapshot("a = 'Hello, '")?;

        let mut engine = Rc::new(Isolate::new());
        engine.init();

        let s = fs::read(SNAPSHOT_PATH)?;
        let m = Rc::get_mut(&mut engine).unwrap();
        (*m).load_snapshot(s.as_slice());

        let r = engine.eval("a = 1;");

        assert!(r.is_ok());

        Ok(())
    }
}