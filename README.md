Recipro
=========

The V8 Javascript Engine from Rust.

# Install

```
$ git clone https://github.com/uzushino/recipro.git

$ cd recipro/engine

# First download and build v8 engine
$ make build

# Then pyodide downlaod and install
$ cd ../../scripts/pyodide/

$ wget https://github.com/iodide-project/pyodide/releases/download/0.12.0/pyodide-build-0.12.0.tar.bz2

$ tar xf pyodide-build-0.12.0.tar.bz2

$ cp ../pyodide.* ./
```

# Usage

```
engine.execute_script(r#"
languagePluginLoader.then(() => {
  console.log(pyodide.runPython('import sys\nsys.version'));
})
.catch(e => Recipro.log(e.stack));
"#);

$ cargo run 
Logged: Python initialization complete
Logged: 3.7.0 (default, May  3 2019, 19:02:54) 
[Clang 6.0.1 ]
```
