extern crate failure;
extern crate recipro_engine;

fn main() -> Result<(), failure::Error> {
  let version = recipro_engine::v8_version();
  println!("version: {}", version);

  recipro_engine::initialize();

  {
    let isolate = recipro_engine::isolate::Isolate::new();

    isolate.execute("'Hello from rust !'".to_string())?;
    isolate.execute("throw 'Error !'".to_string())?;
  }

  recipro_engine::shutdown();

  Ok(())
}
