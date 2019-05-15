extern crate failure;
extern crate recipro_engine;

use recipro_engine::{ Engine, Isolate, Platform, Snapshot, module::Module };

const SNAPSHOT_PATH: &'static str = "/tmp/snapshot";

fn write_snapshot(script: &str) -> Result<(), failure::Error> {
    let engine = Snapshot::new();
    engine.init();
    engine.eval(script.into())?;

    let snapshot = engine.snapshot();
    if snapshot.data_size > 0 {
        std::fs::write(SNAPSHOT_PATH, snapshot.as_slice())?;
        Snapshot::delete_snapshot(snapshot.data_ptr);
    }

    Ok(())
}

fn main() -> Result<(), failure::Error> {
    /*
    Platform::init();
   
    write_snapshot("a = 'Hello, '")?;

    {
        let s = std::fs::read(SNAPSHOT_PATH)?;
        let engine = Isolate::new(Some(s.as_slice()));
        engine.init();

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
    }    

    Platform::shutdown();
*/
    Ok(())
}
