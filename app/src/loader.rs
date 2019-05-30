use std::env;
use std::sync::{ Arc, Mutex };

use futures::{
  Future,
  Async,
  Poll,

  task
};

use recipro_engine::{ Engine, Isolate, Platform };

pub struct Loader {
  platform: Arc<Mutex<Platform>>,
  loading: bool,
}

impl Loader {
  pub fn new() -> Loader {
    Loader {
      platform: Arc::new(Mutex::new(Platform::new())),
      loading: false
    }
  }
  
  fn prepare(platform: &mut Platform, task: task::Task) -> Result<(), failure::Error> {
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

      task.notify();

      Ok(())
  }
}

impl Future for Loader {
  type Item = ();

  type Error = ();

  fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
    if !self.loading {
      let shared = self.platform.clone();
      let current = task::current();

      std::thread::spawn(move || {
        let mut platform = shared.lock().unwrap();
        Self::prepare(&mut platform, current).unwrap();
      });

      self.loading = true;

      return Ok(Async::NotReady);
    } 

    Ok(Async::Ready(()))
  }
}