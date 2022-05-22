use crate::matcher::Matcher;

pub struct Config {
    pub bind_address: String,
    pub upstream_address: String,
    pub max_packet_size: usize,
    pub rules: Vec<(Matcher,)>,
}
