use std::env;
use std::sync::Arc;

use futures::{
  Future,
  Async,
  Poll,
};

use recipro_engine::{ Engine, Isolate, Platform };

pub struct Loader {
  platform: Platform,
}

impl Loader {
  pub fn new() -> Loader {
    Loader {
      platform: Platform::new(),
    }
  }
  
  fn prepare(&mut self) -> Result<(), failure::Error> {
      let engine = Arc::new(Isolate::new());
      self.platform.add_engine(engine.clone());
      
      let scripts_dir = env::current_dir()?
          .join("scripts");
      let pyodide_dir = scripts_dir
          .join("pyodide");

      let s = std::fs::read_to_string(scripts_dir.join("main.js"))?;
      engine.execute_script(s.as_str())?;

      let s = format!("self.languagePluginUrl = '{}/';",  pyodide_dir.to_string_lossy());
      engine.execute_script(s.as_str())?;

      let s = std::fs::read_to_string(pyodide_dir.join("pyodide.js"))?;
      engine.execute_script(s.as_str())?;

      Ok(())
  }

  fn execute_python(&self, python: &str) -> Result<(), failure::Error> {
    let engine = self.platform.engines[0].clone();

    engine.execute_script(format!("(async function() {{
      await languagePluginLoader;
      pyodide.runPython(`{py}`)
    }})()", py = python).as_str())
  }
}

impl Future for Loader {
  type Item = ();

  type Error = ();

  fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
    self.prepare().unwrap();
    Ok(Async::Ready(()))
  }
}