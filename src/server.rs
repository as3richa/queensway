use crate::matcher::Matcher;
use crate::protocol::Message;
use crate::protocol::Record;

use std::error::Error;
use std::io::Error as IoError;
use std::net::SocketAddr;
use std::net::UdpSocket as StdUdpSocket;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use rayon::{ThreadPool, ThreadPoolBuilder};
use tokio::net::UdpSocket;
use tokio::sync::oneshot::channel;
use tokio::sync::Semaphore;
use tokio::task::spawn;
//use tracing::{info, span, Level};
use log::{info, warn};

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

struct Server {
    config: Config,
    thread_pool: ThreadPool,
    socket: UdpSocket,
    semaphore: Semaphore,
}

pub async fn bind_and_serve(config: Config) -> Result<(), Box<dyn Error>> {
    let thread_pool = ThreadPoolBuilder::new().num_threads(1).build()?;

    let socket = bind_socket(
        &config.bind_address,
        config.read_timeout,
        config.write_timeout,
        &thread_pool,
    )
    .await?;

    let semaphore = Semaphore::new(config.max_concurrent_requests);

    info!("Serving DNS queries via UDP on {} and proxying to {}", config.bind_address, config.upstream_address);

    let server = Arc::new(Server {
        config,
        thread_pool,
        socket,
        semaphore,
    });

    loop {
        let server = server.clone();

        let mut buffer = vec![0; server.config.max_packet_size];
        let (len, source_address) = server.socket.recv_from(&mut buffer).await?;

        spawn(async move {
            match serve_request(source_address, buffer, len, server).await {
                Ok(()) => (),
                Err(error) => info!(
                    "Error serving DNS request from {}: {}",
                    source_address, error
                ),
            }
        });
    }
}

async fn bind_socket(
    bind_address: &str,
    read_timeout: Duration,
    write_timeout: Duration,
    thread_pool: &ThreadPool,
) -> Result<UdpSocket, IoError> {
    // Infallible
    let bind_address = String::from_str(bind_address).unwrap();

    let (sender, receiver) = channel::<Result<StdUdpSocket, IoError>>();

    thread_pool.spawn(move || {
        let build_socket = || -> Result<StdUdpSocket, IoError> {
            let std_socket = StdUdpSocket::bind(bind_address)?;
            std_socket.set_read_timeout(Some(read_timeout))?;
            std_socket.set_write_timeout(Some(write_timeout))?;
            Ok(std_socket)
        };

        // Fallible only in the case that the other side has hung up
        sender.send(build_socket()).unwrap();
    });

    // Fallible only in the case that the other side has hung up
    let std_socket = receiver.await.unwrap()?;
    let socket = UdpSocket::from_std(std_socket)?;

    Ok(socket)
}

async fn serve_request(
    source_address: SocketAddr,
    mut buffer: Vec<u8>,
    len: usize,
    server: Arc<Server>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = server.semaphore.acquire().await;


    let query = Message::parse(&buffer[0..len])?;

    info!("Received DNS query from {}:\n{}", source_address, query);

    let upstream_socket = bind_socket(
        &server.config.egress_address,
        server.config.read_timeout,
        server.config.write_timeout,
        &server.thread_pool,
    )
    .await?;

    upstream_socket
        .send_to(&buffer[0..len], &server.config.upstream_address)
        .await?; // FIXME: ???

    let (len, _) = upstream_socket.recv_from(&mut buffer).await?;

    let reply = Message::parse(&buffer[0..len]).unwrap(); // FIXME

    info!(
        "Received DNS reply from {} to query originating from {}:\n{}",
        server.config.upstream_address, source_address, reply
    );

    server
        .socket
        .send_to(&buffer[0..len], &source_address)
        .await?;

    Ok(())
}

async fn transform_query(query: Message) -> (Option<Message>, Vec<Record>) {
    (Some(query), vec![])
}

async fn transform_response(reply: Message) -> Message {
    reply
}
