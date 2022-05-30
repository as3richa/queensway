mod matcher;
mod protocol;
mod server;

use crate::server::{bind_and_serve, Config};

use env_logger::{Builder as LoggerBuilder,Env};

#[tokio::main]
async fn main() {
    LoggerBuilder::from_env(Env::default().default_filter_or("warn")).init();
    let config = Config::default();
    bind_and_serve(config).await.unwrap(); // FIXME
}
