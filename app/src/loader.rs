use std::env;
use std::sync::Arc;

use futures::{
  Future,
  Async,
  Poll
};

use recipro_engine::{ Engine, Isolate, Platform };

pub struct PyodideApp {
  platform: Platform,
}

impl PyodideApp {
  pub fn new() -> PyodideApp {
    PyodideApp {
      platform: Platform::new()
    }
  }

  fn prepare(platform: &mut Platform) -> Result<(), failure::Error> {
      let engine = Arc::new(Isolate::new());
      platform.add_engine(engine.clone());
      
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
}

impl Future for PyodideApp {
  type Item = ();
  type Error = failure::Error;

  fn poll<'a>(&'a mut self) -> Poll<Self::Item, Self::Error> {
    match Self::prepare(&mut self.platform) {
      Ok(_) => Ok(Async::Ready(())),
      Err(_) => Err(failure::err_msg("")),
    }
  }
}

pub struct Loader<T>(pub T);

impl<T> Future for Loader<T> where T: Future {
  type Item = ();

  type Error = ();

  fn poll<'a>(&'a mut self) -> Poll<Self::Item, Self::Error> {
    match self.0.poll() {
      Ok(Async::Ready(_)) => {},
      Ok(Async::NotReady) => return Ok(Async::NotReady),
      Err(_) => return Err(()),
    }

    Ok(Async::Ready(()))
  }
}