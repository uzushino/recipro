use std::rc::Rc;
use recipro_engine::{ Engine, Isolate, Platform };

fn main() -> Result<(), failure::Error> {
    println!("version: {}", Platform::version());

    let mut platform: Platform = Platform::new();
    let engine = Rc::new(Isolate::new());
    platform.add_engine(engine.clone());
    
    let mod_a = engine.compile(
        "a.js", 
        "import b from 'b.js'\nRecipro.log('Rust');\n Recipro.log(b());"
    )?;
    let mod_b = engine.compile(
        "b.js", 
        "export default function () { return 'this is b.js'; };"
    )?;

    engine.instantiate(mod_a, &mut |_s, _id| mod_b);
    engine.evaluate(mod_a);

    Ok(())
}
