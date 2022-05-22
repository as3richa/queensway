use crate::config::Config;

use std::error::Error;
use std::io::Error as IoError;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;

pub async fn bind_and_serve(config: Config) -> Result<(), IoError> {
    let socket = UdpSocket::bind(&config.bind_address).await?;

    loop {
        let mut buf = [0u8; 256 * 1024];
        let (len, source_address) = socket.recv_from(&mut buf).await?;
        println!("{:?} {:?}", len, source_address);
    }
}
