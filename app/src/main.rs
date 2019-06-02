use tokio;

mod loader;

use loader::Loader;

fn main()  {
    let server = Loader::new();
    tokio::run(server);
}
