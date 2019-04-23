extern crate failure;
extern crate recipro_engine;

use recipro_engine::platform::Platform;

fn main() -> Result<(), failure::Error> {
  let version = Platform::version();
  println!("version: {}", version);

  let platform = Platform::new();
  let isolate = platform.isolate();
  isolate.execute("'Hello from rust !'".to_string())?;
  isolate.execute("throw 'Error !'".to_string())?;

  Ok(())
}
