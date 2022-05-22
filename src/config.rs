use crate::matcher::Matcher;
use crate::protocol::Record;

use std::time::Duration;

pub struct Config {
    pub bind_address: String,
    pub source_address: String,
    pub upstream_address: String,
    pub max_packet_size: usize,
    pub read_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
    pub rules: Vec<(Matcher, Vec<Record>)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:53".to_string(),
            source_address: "0.0.0.0:0".to_string(),
            upstream_address: "8.8.8.8:53".to_string(), // Google DNS, FIXME
            max_packet_size: 256 * 1024,
            read_timeout: Some(Duration::from_secs(5)),
            write_timeout: Some(Duration::from_secs(5)),
            rules: vec![],
        }
    }
}
