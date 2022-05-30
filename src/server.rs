use crate::matcher::Matcher;
use crate::protocol::Message;
use crate::protocol::Record;

use std::io::Error as IoError;
use std::net::UdpSocket as StdUdpSocket;
use std::sync::Arc;
use std::time::Duration;

use rayon::ThreadPoolBuilder;
use tokio::net::UdpSocket;
use tokio::sync::oneshot::channel;
use tokio::task::spawn;

pub struct Config {
    pub bind_address: String,
    pub upstream_address: String,
    pub egress_address: String,
    pub max_packet_size: usize,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub max_concurrent_requests: usize,
    pub rules: Vec<(Matcher, Vec<Record>)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:53".to_string(),
            upstream_address: "8.8.8.8:53".to_string(),
            egress_address: "0.0.0.0:0".to_string(),
            max_packet_size: 256 * 1024,
            read_timeout: Duration::from_secs(5),
            write_timeout: Duration::from_secs(5),
            max_concurrent_requests: 100,
            rules: vec![],
        }
    }
}

pub struct Server {
    config: Config,
    socket: UdpSocket,
}

impl Server {
    pub fn bind(config: Config) -> Result<Self, IoError> {
        let socket = {
            let std_socket = StdUdpSocket::bind(&config.bind_address)?;
            std_socket.set_read_timeout(Some(config.read_timeout))?;
            std_socket.set_write_timeout(Some(config.write_timeout))?;
            UdpSocket::from_std(std_socket)?
        };

        Ok(Server { config, socket })
    }

    pub async fn serve(self) -> Result<(), IoError> {
        let Server { config, socket } = self;

        let Config {
            egress_address,
            upstream_address,
            read_timeout,
            write_timeout,
            ..
        } = config;

        let upstream_socket_thread_pool =
            Arc::new(ThreadPoolBuilder::new().num_threads(1).build().unwrap()); // FIXME

        loop {
            let mut buf = vec![0; config.max_packet_size];
            let (len, source_address) = socket.recv_from(&mut buf).await?;
            let message = Message::parse(&buf[0..len]).unwrap(); // FIXME
            println!("{:?} {}", source_address, message);

            let egress_address = egress_address.clone();
            let upstream_address = upstream_address.clone();
            let upstream_socket_thread_pool = upstream_socket_thread_pool.clone();

            spawn(async move {
                let (sender, receiver) = channel();

                upstream_socket_thread_pool.spawn(move || {
                    let build_socket = || -> Result<StdUdpSocket, IoError> {
                        let std_socket = StdUdpSocket::bind(egress_address)?;
                        std_socket.set_read_timeout(Some(read_timeout))?;
                        std_socket.set_write_timeout(Some(write_timeout))?;
                        Ok(std_socket)
                    };

                    // Fallible only in the case that the other side has hung up
                    sender.send(build_socket()).unwrap();
                });

                let upstream_socket = {
                    let std_socket = receiver.await.unwrap()?;
                    UdpSocket::from_std(std_socket)? // FIXME
                };

                upstream_socket
                    .send_to(&buf[0..len], &upstream_address)
                    .await?; // FIXME: ???

                let (len, _) = upstream_socket.recv_from(&mut buf).await?;

                let message = Message::parse(&buf[0..len]).unwrap(); // FIXME
                println!("{}", message);
                Ok::<(), IoError>(()) // FIXME: ???
            });
        }
    }
}
