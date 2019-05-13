extern crate failure;
extern crate recipro_engine;

use std::ffi::CString;

use recipro_engine::{ Engine, Isolate, Platform, Snapshot, module::Module };

const SNAPSHOT_PATH: &'static str = "/tmp/snapshot";

fn write_snapshot(script: &str) -> Result<(), failure::Error> {
    let engine = Snapshot::new();
    let platform = Platform::new(&engine);
    platform.engine_start();

    engine.eval(script.into())?;

    let snapshot = engine.snapshot();
    if snapshot.data_size > 0 {
        std::fs::write(SNAPSHOT_PATH, snapshot.as_slice())?;
        Snapshot::delete_snapshot(snapshot.data_ptr);
    }

    Ok(())
}

fn main() -> Result<(), failure::Error> {
    let version = Platform::version();
    println!("version: {}", version);

    write_snapshot("a = 'Hello, '")?;

    let s = std::fs::read(SNAPSHOT_PATH)?;
    let engine = Isolate::new(s.as_slice());
    let platform = Platform::new(&engine);
    platform.engine_start();

    let vm = engine.core();
    let name = CString::new("a.js")?;
    let script = CString::new("import { b } from 'b.js'\n1 + 1;")?;
    let id = Module::compile(vm, name.as_ptr(), script.as_ptr());
    println!("compile id {}", id);

    let a = &mut |_s, id| {
        println!("specifier\n");
        id
    };
    Module::instantiate(vm, id, a);

    Ok(())
}
