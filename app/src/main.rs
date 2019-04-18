extern crate recipro_engine;

fn main() {
  let version = recipro_engine::v8_version();
  println!("version: {}", version);

  recipro_engine::init();
 
  recipro_engine::eval("'Hello from rust !'".to_string());
  recipro_engine::eval("'Error !".to_string());
  
  recipro_engine::dispose();
  recipro_engine::shutdown_platform();
}
