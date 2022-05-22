use crate::config::Config;
use crate::protocol::Message;

use std::io::Error as IoError;
use std::net::UdpSocket as StdUdpSocket;
use tokio::net::UdpSocket;

pub struct Server {
    config: Config,
    socket: UdpSocket,
}

impl Server {
    pub fn bind(config: Config) -> Result<Self, IoError> {
        let std_socket = StdUdpSocket::bind(&config.bind_address)?;
        std_socket.set_read_timeout(config.read_timeout)?;
        std_socket.set_write_timeout(config.write_timeout)?;

        let socket = UdpSocket::from_std(std_socket)?;

        Ok(Server { config, socket })
    }

    pub async fn serve(self) -> Result<(), IoError> {
        let Server { config, socket } = self;
        let mut buf = vec![0; config.max_packet_size];

        loop {
            let (len, source_address) = socket.recv_from(&mut buf).await?;
            println!("{:?} {:?}", len, source_address);

            let message = Message::parse(&buf[0..len]);
            println!("{:?}", message);
        }
    }
}
