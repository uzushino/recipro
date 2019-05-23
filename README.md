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