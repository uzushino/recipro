use std::env;
use std::sync::Arc;
use recipro_engine::{ Engine, Isolate, Platform };

fn main() -> Result<(), failure::Error> {
    let mut platform: Platform = Platform::new();
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

    engine.execute_script(r#"
const python = `
import io
import os
import pandas as pd

csv = """
A,B,C
1,11,111
2,22,222
3,33,333
"""

df = pd.read_csv(io.StringIO(csv), index_col="A")
print(df)
`;
  
pyodide.loadPackage([
    "pytz",
    "python-dateutil",
    "pandas"
]);

async function __main__() {
  await languagePluginLoader;
  pyodide.runPython(python);
}

__main__();
"#)?;

    Ok(())
}
