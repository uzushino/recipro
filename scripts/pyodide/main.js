navigator = { userAgent: 'recipro' };
self = {};
Module = {};

require = Recipro.runScript;
readFileAsync = Recipro.readFile

console = {
  log: Recipro.log,
  warn: Recipro.log,
  err: Recipro.log
};

function fetch(file) {
  return new Promise(resolve => {
      var buf = Recipro.readFile(file);
      resolve(buf);
  });
}