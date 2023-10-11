use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordConfig {
    pub token: String,
    pub prefix: String,
    pub owners: Vec<u64>,
    pub guild: u64,
    pub edit_track_timespan: u64,
}