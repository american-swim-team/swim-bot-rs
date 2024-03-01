use serde::Deserialize;
use log::LevelFilter;

// Custom deserializer function
fn deserialize_level_filter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "off" => Ok(LevelFilter::Off),
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "debug" => Ok(LevelFilter::Debug),
        "trace" => Ok(LevelFilter::Trace),
        _ => Ok(LevelFilter::Info),
    } 
}

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    #[serde(deserialize_with = "deserialize_level_filter")]
    pub level: LevelFilter,
    pub file_output: String,
    pub stdout: bool,
}