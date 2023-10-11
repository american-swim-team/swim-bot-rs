use std::net::SocketAddr;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct APIConfig {
    pub address: String,
    pub port: u16,
}

impl APIConfig {
    pub fn address(&self) -> SocketAddr {
        // Setup SocketAddr using address and port
        format!("{}:{}", self.address, self.port)
            .parse()
            .expect("Invalid address")
    }
}