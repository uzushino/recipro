extern crate failure;
extern crate recipro_engine;

use recipro_engine::{ 
  platform::Platform,
  isolate::Snapshot
};

fn main() -> Result<(), failure::Error> {
  let version = Platform::version();
  println!("version: {}", version);
    
  let snapshot: *mut Snapshot = {
    let platform = Platform::new_snapshot();
    let isolate = platform.isolate();
      
    isolate.execute("a = 'Hello, '".to_string())?;
    isolate.snapshot()
  };

  unsafe {
    {
      let platform = Platform::new(snapshot);
      let isolate = platform.isolate();

      isolate.execute("a + 'Rust !'".to_string())?;
    }

    recipro_engine::isolate::delete_snapshot(snapshot);
  }

  Ok(())
}
