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
const python = `
from datetime import datetime
print(datetime.now())

import itertools
acc = itertools.accumulate([1,2,3,4,5,6,7,8,9,10])
print(list(acc))
`;

languagePluginLoader.then(() => {
  console.log(pyodide.runPython(python));
})
.catch(e => console.log(e.stack));
"#)?;

    Ok(())
}
