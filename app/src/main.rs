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
    let platform = Platform::new(std::ptr::null_mut());
    let isolate = platform.isolate();

    isolate.execute("a = 1".to_string())?;
    isolate.snapshot()
  };

  unsafe {
    println!("snapshot size: {}", (* snapshot).data_size);
    recipro_engine::isolate::delete_snapshot(snapshot);
  }

  Ok(())
}
