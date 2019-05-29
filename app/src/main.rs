use tokio;

mod loader;

use loader::{ Loader, PyodideApp };

fn main()  {
    let server = Loader(PyodideApp::new());
    tokio::run(server);
}
