extern crate failure;
extern crate recipro_engine;

fn main() -> Result<(), failure::Error> {
  let version = recipro_engine::v8_version();
  println!("version: {}", version);

  recipro_engine::initialize();

  recipro_engine::execute_script("'Hello from rust !'".to_string())?;
  recipro_engine::execute_script("throw 'Error !'".to_string())?;

  recipro_engine::shutdown();

  Ok(())
}
