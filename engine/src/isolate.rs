use std::ffi::CString;
use std::cell::RefCell;
use std::ops::Deref;

use libc::{ c_void, c_int, c_char };

use crate::{Engine, Isolate, ReciproVM, Snapshot};

pub type ModId = i32;
type Callback = extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int;
type Closure<'a> = dyn FnMut(*mut c_char, c_int) -> c_int + 'a;

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

        pub fn module_compile(vm: *mut ReciproVM, filename: *const u8, script: *const c_char) -> ModId;
        pub fn module_instantiate(vm: *mut ReciproVM, id: ModId, data: *mut c_void, f: Callback);
        pub fn module_evaluate(vm: *mut ReciproVM, id: ModId) -> *const ModId;
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

    pub fn compile_from_script(&self, filename: &str) -> Result<ModId, failure::Error> {
        let source = std::fs::read_to_string(filename)?;
        let script = CString::new(source)?;
        let id = unsafe { 
            ffi::module_compile(
                self.core(), 
                filename.as_ptr(), 
                script.as_ptr()
            ) 
        };

        Ok(id)
    }

    pub fn compile(&self, filename: &str, script: &str) -> Result<ModId, failure::Error> {
        let script = CString::new(script.as_bytes())?;
        let id = unsafe { 
            ffi::module_compile(
                self.core(), 
                filename.as_ptr(), 
                script.as_ptr()
            ) 
        };

        Ok(id)
    }

    pub fn instantiate(&self, id: ModId, closure: &mut Closure) {
        let mut closure_ptr = Box::new(closure);

        unsafe { 
        ffi::module_instantiate(
            self.core(), 
            id, 
            &mut *closure_ptr as *mut _  as *mut c_void, 
            Self::resolve_callback
        );
        };
    }

    pub fn evaluate(&self, id: ModId) -> Result<(), failure::Error> {
        unsafe { 
            ffi::module_evaluate(
                self.core(), 
                id
            ) 
        };
        Ok(())
    }

    extern "C" fn resolve_callback(data: *mut c_void, specifier: *mut c_char, id: c_int) -> c_int {
        unsafe {
        let closure: &mut &mut Closure =  &mut *(data as *mut _);
        closure(specifier, id)
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

    #[test]
    fn evaluate_module() -> Result<(), failure::Error> {
        init_platform();

        let engine = Isolate::new();
        engine.init();

        let mod_a = engine.compile(
            "a.js", 
            "import b from 'b.js'\nRecipro.log('Rust');\n Recipro.log(b());"
        )?;
        let mod_b = engine.compile(
            "b.js", 
            "export default function () { return 'this is b.js'; };"
        )?;

        engine.instantiate(mod_a, &mut |_s, _id| mod_b);
        engine.evaluate(mod_a)?;

        Ok(())
    }
}