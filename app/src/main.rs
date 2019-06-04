use std::sync::Arc;
use recipro_engine::{ Engine, Isolate, Platform };

fn main() -> Result<(), failure::Error> {
    let mut platform = Platform::new();
    let engine = Arc::new(Isolate::new());
    platform.add_engine(engine.clone());

    engine.execute_script("a = 1 + 1;")?;
    engine.execute_script("Recipro.log(a);")?;

    Ok(())
}
