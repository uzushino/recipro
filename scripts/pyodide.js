/**
 * The main bootstrap script for loading pyodide.
 */
languagePluginLoader = new Promise((resolve, reject) => {
  // This is filled in by the Makefile to be either a local file or the
  // deployed location. TODO: This should be done in a less hacky
  // way.
  var baseURL = self.languagePluginUrl || 'https://iodide.io/pyodide-demo/';
  baseURL = baseURL.substr(0, baseURL.lastIndexOf('/')) + '/';

  ////////////////////////////////////////////////////////////
  // Package loading
  let loadedPackages = [];
  var loadPackagePromise = new Promise((resolve) => resolve());
  // Regexp for validating package name and URI
  var package_name_regexp = '[a-z0-9_][a-z0-9_\-]*'
  var package_uri_regexp =
      new RegExp('^https?://.*?(' + package_name_regexp + ').js$', 'i');
  var package_name_regexp = new RegExp('^' + package_name_regexp + '$', 'i');

  let _uri_to_package_name = (package_uri) => {
    // Generate a unique package name from URI

    if (package_name_regexp.test(package_uri)) {
      return package_uri;
    } else if (package_uri_regexp.test(package_uri)) {
      let match = package_uri_regexp.exec(package_uri);
      // Get the regexp group corresponding to the package name
      return match[1];
    } else {
      return null;
    }
  };

  // clang-format off
  let preloadWasm = () => {
    // On Chrome, we have to instantiate wasm asynchronously. Since that
    // can't be done synchronously within the call to dlopen, we instantiate
    // every .so that comes our way up front, caching it in the
    // `preloadedWasm` dictionary.

    let promise = new Promise((resolve) => resolve());
    let FS = Module.FS;

    function recurseDir(rootpath) {
      let dirs;
      try {
        dirs = FS.readdir(rootpath);
      } catch {
        return;
      }
      for (let entry of dirs) {
        if (entry.startsWith('.')) {
          continue;
        }
        const path = rootpath + entry;
        if (entry.endsWith('.so')) {
          if (Module['preloadedWasm'][path] === undefined) {
            promise = promise
              .then(() => Module['loadWebAssemblyModule'](
                FS.readFile(path), {loadAsync: true}))
              .then((module) => {
                Module['preloadedWasm'][path] = module;
              });
          }
        } else if (FS.isDir(FS.lookupPath(path).node.mode)) {
          recurseDir(path + '/');
        }
      }
    }

    recurseDir('/');

    return promise;
  }
  // clang-format on

  /* CHANGE 
  function loadScript(url, onload, onerror) {
    if (self.document) { // browser
      const script = self.document.createElement('script');
      script.src = url;
      script.onload = (e) => { onload(); };
      script.onerror = (e) => { onerror(); };
      self.document.head.appendChild(script);
    } else if (self.importScripts) { // webworker
      try {
        self.importScripts(url);
        onload();
      } catch {
        onerror();
      }
    }
  }
  */
  function loadScript(url, onload, onerror) {
    require(url);
    onload();
  }

  let _loadPackage = (names, messageCallback) => {
    // DFS to find all dependencies of the requested packages
    let packages = self.pyodide._module.packages.dependencies;
    let loadedPackages = self.pyodide.loadedPackages;
    let queue = [].concat(names || []);
    let toLoad = new Array();
    while (queue.length) {
      let package_uri = queue.pop();

      const package = _uri_to_package_name(package_uri);

      if (package == null) {
        console.error(`Invalid package name or URI '${package_uri}'`);
        return;
      } else if (package == package_uri) {
        package_uri = 'default channel';
      }

      if (package in loadedPackages) {
        if (package_uri != loadedPackages[package]) {
          console.error(`URI mismatch, attempting to load package ` +
                        `${package} from ${package_uri} while it is already ` +
                        `loaded from ${loadedPackages[package]}!`);
          return;
        }
      } else if (package in toLoad) {
        if (package_uri != toLoad[package]) {
          console.error(`URI mismatch, attempting to load package ` +
                        `${package} from ${package_uri} while it is already ` +
                        `being loaded from ${toLoad[package]}!`);
          return;
        }
      } else {
        console.log(`Loading ${package} from ${package_uri}`);

        toLoad[package] = package_uri;
        if (packages.hasOwnProperty(package)) {
          packages[package].forEach((subpackage) => {
            if (!(subpackage in loadedPackages) && !(subpackage in toLoad)) {
              queue.push(subpackage);
            }
          });
        } else {
          console.error(`Unknown package '${package}'`);
        }
      }
    }

    self.pyodide._module.locateFile = (path) => {
      // handle packages loaded from custom URLs
      let package = path.replace(/\.data$/, "");
      if (package in toLoad) {
        let package_uri = toLoad[package];
        if (package_uri != 'default channel') {
          return package_uri.replace(/\.js$/, ".data");
        };
      };
      return baseURL + path;
    };

    let promise = new Promise((resolve, reject) => {
      if (Object.keys(toLoad).length === 0) {
        resolve('No new packages to load');
        return;
      }

      const packageList = Array.from(Object.keys(toLoad)).join(', ');
      if (messageCallback !== undefined) {
        messageCallback(`Loading ${packageList}`);
      }

      // monitorRunDependencies is called at the beginning and the end of each
      // package being loaded. We know we are done when it has been called
      // exactly "toLoad * 2" times.
      var packageCounter = Object.keys(toLoad).length * 2;

      self.pyodide._module.monitorRunDependencies = () => {
        packageCounter--;
        if (packageCounter === 0) {
          for (let package in toLoad) {
            self.pyodide.loadedPackages[package] = toLoad[package];
          }
          delete self.pyodide._module.monitorRunDependencies;
          //** REMOVE self.removeEventListener('error', windowErrorHandler); */
          if (!isFirefox) {
            preloadWasm().then(() => {resolve(`Loaded ${packageList}`)});
          } else {
            resolve(`Loaded ${packageList}`);
          }
        }
      };

      // Add a handler for any exceptions that are thrown in the process of
      // loading a package
      var windowErrorHandler = (err) => {
        delete self.pyodide._module.monitorRunDependencies;
        self.removeEventListener('error', windowErrorHandler);
        // Set up a new Promise chain, since this one failed
        loadPackagePromise = new Promise((resolve) => resolve());
        reject(err.message);
      };
      /** self.addEventListener('error', windowErrorHandler); */

      for (let package in toLoad) {
        let scriptSrc;
        let package_uri = toLoad[package];
        if (package_uri == 'default channel') {
          scriptSrc = `${baseURL}${package}.js`;
        } else {
          scriptSrc = `${package_uri}`;
        }
        loadScript(scriptSrc, () => {}, () => {
          // If the package_uri fails to load, call monitorRunDependencies twice
          // (so packageCounter will still hit 0 and finish loading), and remove
          // the package from toLoad so we don't mark it as loaded.
          console.error(`Couldn't load package from URL ${scriptSrc}`)
          let index = toLoad.indexOf(package);
          if (index !== -1) {
            toLoad.splice(index, 1);
          }
          for (let i = 0; i < 2; i++) {
            self.pyodide._module.monitorRunDependencies();
          }
        });
      }

      // We have to invalidate Python's import caches, or it won't
      // see the new files. This is done here so it happens in parallel
      // with the fetching over the network.
      self.pyodide.runPython('import importlib as _importlib\n' +
                             '_importlib.invalidate_caches()\n');
    });

    return promise;
  };

  let loadPackage = (names, messageCallback) => {
    /* We want to make sure that only one loadPackage invocation runs at any
     * given time, so this creates a "chain" of promises. */
    loadPackagePromise =
        loadPackagePromise.then(() => _loadPackage(names, messageCallback));
    return loadPackagePromise;
  };

  ////////////////////////////////////////////////////////////
  // Fix Python recursion limit
  function fixRecursionLimit(pyodide) {
    // The Javascript/Wasm call stack may be too small to handle the default
    // Python call stack limit of 1000 frames. This is generally the case on
    // Chrom(ium), but not on Firefox. Here, we determine the Javascript call
    // stack depth available, and then divide by 50 (determined heuristically)
    // to set the maximum Python call stack depth.

    let depth = 0;
    function recurse() {
      depth += 1;
      recurse();
    }
    try {
      recurse();
    } catch (err) {
      ;
    }

    let recursionLimit = depth / 50;
    if (recursionLimit > 1000) {
      recursionLimit = 1000;
    }
    pyodide.runPython(
        `import sys; sys.setrecursionlimit(int(${recursionLimit}))`);
  };

  ////////////////////////////////////////////////////////////
  // Rearrange namespace for public API
  let PUBLIC_API = [
    'globals',
    'loadPackage',
    'loadedPackages',
    'pyimport',
    'repr',
    'runPython',
    'runPythonAsync',
    'checkABI',
    'version',
  ];

  function makePublicAPI(module, public_api) {
    var namespace = {_module : module};
    for (let name of public_api) {
      namespace[name] = module[name];
    }
    return namespace;
  }

  ////////////////////////////////////////////////////////////
  // Loading Pyodide
  let wasmURL = `${baseURL}pyodide.asm.wasm`;
  // let Module = {}; * REMOVE *
  self.Module = Module;

  Module.noImageDecoding = true;
  Module.noAudioDecoding = true;
  Module.noWasmDecoding = true;
  Module.preloadedWasm = {};
  let isFirefox = navigator.userAgent.toLowerCase().indexOf('firefox') > -1;

  /** REMOVE 
  let wasm_promise;
  if (WebAssembly.compileStreaming === undefined) {
    wasm_promise = fetch(wasmURL)
                       .then(response => response.arrayBuffer())
                       .then(bytes => WebAssembly.compile(bytes));
  } else {
    wasm_promise = WebAssembly.compileStreaming(fetch(wasmURL));
  }
  */

  Module.instantiateWasm = (info, receiveInstance) => {
    /** UPDATE
    wasm_promise.then(module => WebAssembly.instantiate(module, info))
        .then(instance => receiveInstance(instance))
        .catch(e => console.log(e.stack));
    return {};
    */
    let wasm = readFileAsync(wasmURL)
    let module = new WebAssembly.Module(wasm)
    let instance = new WebAssembly.Instance(module, info)
    receiveInstance(instance);
    return instance.exports;
  };

  Module.checkABI = function(ABI_number) {
    if (ABI_number !== parseInt('1')) {
      var ABI_mismatch_exception =
          `ABI numbers differ. Expected 1, got ${ABI_number}`;
      console.error(ABI_mismatch_exception);
      throw ABI_mismatch_exception;
    } 
    return true;
  };

  /** ADD scripts/pyodide/package.json */
  const packageJson = 
    {"dependencies": {"scikit-learn": ["numpy", "scipy", "joblib"], "sympy": ["mpmath"], "pyparsing": [], "python-dateutil": [], "pluggy": [], "beautifulsoup4": ["soupsieve"], "mpmath": [], "more-itertools": [], "webencodings": [], "mne": ["numpy", "scipy"], "pytest": ["atomicwrites", "attrs", "more-itertools", "pluggy", "py", "setuptools"], "attrs": [], "networkx": ["decorator", "setuptools", "matplotlib", "numpy"], "nose": ["setuptools"], "pandas": ["numpy", "python-dateutil", "pytz"], "numpy": [], "atomicwrites": [], "bleach": ["setuptools", "webencodings"], "biopython": [], "py": [], "joblib": [], "decorator": [], "matplotlib": ["cycler", "kiwisolver", "numpy", "pyparsing", "python-dateutil", "pytz"], "docutils": [], "cycler": [], "kiwisolver": [], "scipy": ["numpy"], "pytz": [], "distlib": [], "micropip": ["distlib"], "html5lib": ["webencodings"], "Jinja2": ["MarkupSafe"], "setuptools": ["pyparsing"], "MarkupSafe": [], "soupsieve": [], "Pygments": [], "test": []}, "import_name_to_package_name": {"sklearn": "scikit-learn", "sklearn.calibration": "scikit-learn", "sklearn.cluster": "scikit-learn", "sklearn.compose": "scikit-learn", "sklearn.covariance": "scikit-learn", "sklearn.cross_decomposition": "scikit-learn", "sklearn.datasets": "scikit-learn", "sklearn.decomposition": "scikit-learn", "sklearn.discriminant_analysis": "scikit-learn", "sklearn.dummy": "scikit-learn", "sklearn.ensemble": "scikit-learn", "sklearn.exceptions": "scikit-learn", "sklearn.externals": "scikit-learn", "sklearn.feature_extraction": "scikit-learn", "sklearn.feature_selection": "scikit-learn", "sklearn.gaussian_process": "scikit-learn", "sklearn.impute": "scikit-learn", "sklearn.isotonic": "scikit-learn", "sklearn.kernel_approximation": "scikit-learn", "sklearn.kernel_ridge": "scikit-learn", "sklearn.linear_model": "scikit-learn", "sklearn.manifold": "scikit-learn", "sklearn.metrics": "scikit-learn", "sklearn.mixture": "scikit-learn", "sklearn.model_selection": "scikit-learn", "sklearn.multiclass": "scikit-learn", "sklearn.multioutput": "scikit-learn", "sklearn.naive_bayes": "scikit-learn", "sklearn.neighbors": "scikit-learn", "sklearn.neural_network": "scikit-learn", "sklearn.pipeline": "scikit-learn", "sklearn.preprocessing": "scikit-learn", "sklearn.random_projection": "scikit-learn", "sklearn.semi_supervised": "scikit-learn", "sklearn.svm": "scikit-learn", "sklearn.tree": "scikit-learn", "sklearn.utils": "scikit-learn", "sympy": "sympy", "pyparsing": "pyparsing", "dateutil": "python-dateutil", "pluggy": "pluggy", "bs4": "beautifulsoup4", "mpmath": "mpmath", "more_itertools": "more-itertools", "webencodings": "webencodings", "mne": "mne", "pytest": "pytest", "attr": "attrs", "networkx": "networkx", "networkx.algorithms": "networkx", "networkx.algorithms.approximation": "networkx", "networkx.algorithms.assortativity": "networkx", "networkx.algorithms.bipartite": "networkx", "networkx.algorithms.centrality": "networkx", "networkx.algorithms.chordal": "networkx", "networkx.algorithms.coloring": "networkx", "networkx.algorithms.community": "networkx", "networkx.algorithms.components": "networkx", "networkx.algorithms.connectivity": "networkx", "networkx.algorithms.flow": "networkx", "networkx.algorithms.isomorphism": "networkx", "networkx.algorithms.link_analysis": "networkx", "networkx.algorithms.node_classification": "networkx", "networkx.algorithms.operators": "networkx", "networkx.algorithms.shortest_paths": "networkx", "networkx.algorithms.traversal": "networkx", "networkx.algorithms.tree": "networkx", "networkx.classes": "networkx", "networkx.drawing": "networkx", "networkx.generators": "networkx", "networkx.linalg": "networkx", "networkx.readwrite": "networkx", "networkx.readwrite.json_graph": "networkx", "networkx.utils": "networkx", "nose": "nose", "pandas": "pandas", "numpy": "numpy", "atomicwrites": "atomicwrites", "bleach": "bleach", "Bio": "biopython", "py": "py", "py.code": "py", "joblib": "joblib", "decorator": "decorator", "matplotlib": "matplotlib", "mpl_toolkits": "matplotlib", "docutils": "docutils", "cycler": "cycler", "kiwisolver": "kiwisolver", "scipy": "scipy", "scipy.cluster": "scipy", "scipy.constants": "scipy", "scipy.fftpack": "scipy", "scipy.odr": "scipy", "scipy.sparse": "scipy", "scipy.interpolate": "scipy", "scipy.integrate": "scipy", "scipy.linalg": "scipy", "scipy.misc": "scipy", "scipy.ndimage": "scipy", "scipy.spatial": "scipy", "scipy.special": "scipy", "pytz": "pytz", "distlib": "distlib", "micropip": "micropip", "html5lib": "html5lib", "jinja2": "Jinja2", "setuptools": "setuptools", "easy_install": "setuptools", "pkg_resources": "setuptools", "markupsafe": "MarkupSafe", "soupsieve": "soupsieve", "pygments": "Pygments"}}

  Module.locateFile = (path) => baseURL + path;
  var postRunPromise = new Promise((resolve, reject) => {
    Module.postRun = () => {
      delete self.Module;
      fetch(`${baseURL}packages.json`)
          /** .then((response) => response.json()) */
          .then((json) => {
            fixRecursionLimit(self.pyodide);
            self.pyodide.globals =
                self.pyodide.runPython('import sys\nsys.modules["__main__"]');
            self.pyodide = makePublicAPI(self.pyodide, PUBLIC_API);
            /** UPDATE */
            self.pyodide._module.packages = packageJson;
            resolve();
          })
          .catch(e => console.log(e.stack));
    };
  });

  var dataLoadPromise = new Promise((resolve, reject) => {
    Module.monitorRunDependencies =
        (n) => {
          if (n === 0) {
            delete Module.monitorRunDependencies;
            resolve();
          }
        }
  });

  Promise.all([ postRunPromise, dataLoadPromise ]).then(() => resolve());

  const data_script_src = `${baseURL}pyodide.asm.data.js`;
  Recipro.runScript(data_script_src); 
  
  const scriptSrc = `${baseURL}pyodide.asm.js`;
  Recipro.runScript(scriptSrc);

  self.pyodide = pyodide(Module);
  self.pyodide.loadedPackages = new Array();
  self.pyodide.loadPackage = loadPackage

  ////////////////////////////////////////////////////////////
  // Iodide-specific functionality, that doesn't make sense
  // if not using with Iodide.
  if (self.iodide !== undefined) {
    // Load the custom CSS for Pyodide
    let link = document.createElement('link');
    link.rel = 'stylesheet';
    link.type = 'text/css';
    link.href = `${baseURL}renderedhtml.css`;
    document.getElementsByTagName('head')[0].appendChild(link);

    // Add a custom output handler for Python objects
    self.iodide.addOutputRenderer({
      shouldRender : (val) => {
        return (typeof val === 'function' &&
                pyodide._module.PyProxy.isPyProxy(val));
      },

      render : (val) => {
        let div = document.createElement('div');
        div.className = 'rendered_html';
        var element;
        if (val._repr_html_ !== undefined) {
          let result = val._repr_html_();
          if (typeof result === 'string') {
            div.appendChild(new DOMParser()
                                .parseFromString(result, 'text/html')
                                .body.firstChild);
            element = div;
          } else {
            element = result;
          }
        } else {
          let pre = document.createElement('pre');
          pre.textContent = val.toString();
          div.appendChild(pre);
          element = div;
        }
        return element.outerHTML;
      }
    });
  }
});
languagePluginLoader
/** ADD */
pyodide = self.pyodide
