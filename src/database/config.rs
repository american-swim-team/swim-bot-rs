use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}