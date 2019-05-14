extern crate failure;
extern crate recipro_engine;

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

    let mod_a = Module::new(vm);

    mod_a.compile(
        "a.js", 
        "import b from 'b.js'\nRecipro.log(a + 'Rust');\nRecipro.log(b());"
    )?;

    let mod_b = Module::new(vm);
    mod_b.compile(
        "b.js", 
        "export default function () { return 'this is b.js'; };"
    )?;

    mod_a.instantiate(&mut |_s, _id| mod_b.module_id());
    mod_a.evaluate();

    Ok(())
}
