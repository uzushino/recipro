extern crate failure;
extern crate recipro_engine;

use recipro_engine::{ 
  Platform,
  Snapshot, Isolate, Engine
};

fn main() -> Result<(), failure::Error> {
  let version = Platform::version();
  println!("version: {}", version);
    
  let snapshot = {
    let engine = Snapshot::new();
    Platform::new(&engine).engine_start();    
      
    engine.execute("a = 'Hello, '".to_string())?;
    engine.snapshot()
  };

  {
    let s: &[u8] = unsafe {
      std::slice::from_raw_parts(
        snapshot.data_ptr, 
        snapshot.data_size
      )
    };
    let engine = Isolate::new(s);
    Platform::new(&engine).engine_start();

    engine.execute("a + 'Rust !'".to_string())?;
  }

  Ok(())
}
