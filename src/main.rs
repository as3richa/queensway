mod config;
mod matcher;
mod protocol;
mod server;

use config::Config;
use server::bind_and_serve;

#[tokio::main]
async fn main() {
    let config = Config {
        bind_address: "127.0.0.1:8999".to_string(),
        upstream_address: "8.8.8.8".to_string(),
        max_packet_size: 256 * 1024,
        rules: vec![],
    };

    bind_and_serve(config).await.expect("Oops!");
}
