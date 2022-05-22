mod config;
mod matcher;
mod protocol;
mod server;

use config::Config;
use server::Server;

#[tokio::main]
async fn main() {
    let config = Config::default();
    let server = Server::bind(config).expect("Oops!"); // FIXME
    server.serve().await.expect("Oops!"); // FIXME
}
