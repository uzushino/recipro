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
const fizzbuzz = `
  for i in range(0, 100+1):
    if (i%3) == 0:
      if (i%5) == 0:
        print('FizzBuzz')
      else:
        print('Fizz')
    else:
      if (i%5) == 0:
        print('Buzz')
      else:
        print(i)
`;

async function __main__() {
  await languagePluginLoader;
  pyodide.runPython(fizzbuzz);
}

__main__();
"#)?;

    Ok(())
}
