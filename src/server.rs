use crate::config::Config;
use crate::protocol::Message;

use std::io::Error as IoError;
use std::net::{SocketAddr, UdpSocket as StdUdpSocket};
use std::str::FromStr;
use tokio::net::UdpSocket;

pub struct Server {
    config: Config,
    socket: UdpSocket,
    upstream_socket: UdpSocket,
}

impl Server {
    pub fn bind(config: Config) -> Result<Self, IoError> {
        let socket = {
            let std_socket = StdUdpSocket::bind(&config.bind_address)?;
            std_socket.set_read_timeout(config.read_timeout)?;
            std_socket.set_write_timeout(config.write_timeout)?;
            UdpSocket::from_std(std_socket)?
        };

        let upstream_socket = {
            let std_socket = StdUdpSocket::bind(&config.source_address)?;
            std_socket.set_read_timeout(config.read_timeout)?;
            std_socket.set_write_timeout(config.write_timeout)?;
            UdpSocket::from_std(std_socket)?
        };

        Ok(Server {
            config,
            socket,
            upstream_socket,
        })
    }

    pub async fn serve(self) -> Result<(), IoError> {
        let Server {
            config,
            socket,
            upstream_socket,
        } = self;
        let mut buf = vec![0; config.max_packet_size];

        let upstream_address = SocketAddr::from_str(&config.upstream_address).unwrap(); // FIXME

        loop {
            let (len, source_address) = socket.recv_from(&mut buf).await?;
            println!("{:?} {:?}", len, source_address);

            let message = Message::parse(&buf[0..len]).unwrap(); // FIXME
            println!("{:?}", message);

            println!("{:?}", upstream_address);

            upstream_socket
                .send_to(&buf[0..len], &upstream_address)
                .await?; // FIXME: ???

            let (len, upstream_address) = upstream_socket.recv_from(&mut buf).await?;
            println!("{:?} {:?}", len, upstream_address);

            let message = Message::parse(&buf[0..len]).unwrap(); // FIXME
            println!("{:?}", message);

            println!("\n\n\n\n")
        }
    }
}
