use std::env;
use std::rc::Rc;
use recipro_engine::{ Engine, Isolate, Platform };

fn main() -> Result<(), failure::Error> {
    let mut platform: Platform = Platform::new();
    let engine = Rc::new(Isolate::new());
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

    engine.execute_script(r#"
languagePluginLoader.then(() => {
  console.log(pyodide.runPython('import sys\nsys.version'));
})
.catch(e => Recipro.log(e.stack));
"#)?;

    Ok(())
}
