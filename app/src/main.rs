use std::rc::Rc;
use recipro_engine::{ Engine, Isolate, Platform, module::Module };

fn main() -> Result<(), failure::Error> {
    println!("version: {}", Platform::version());

    let mut platform: Platform = Platform::new();
    let engine = Rc::new(Isolate::new());
    platform.add_engine(engine.clone());
    
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
