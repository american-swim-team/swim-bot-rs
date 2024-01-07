use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::fmt;

use crate::Config;
use crate::database::Database;
use serenity::http::Http;
use warp::reject::Reject;

use poise::serenity_prelude as serenity;

#[derive(Debug, Clone)]
pub struct AppState {
    pub discord: Arc<Http>,
    pub database: Arc<Database>,
    pub config: Config,
}

#[derive(Debug, Deserialize)]
pub struct CheckSteamid {
    pub roles: Vec<i64>,
    pub steamid: i64,
}

#[derive(Debug, Deserialize)]
pub struct ScoreRequest {
    pub steamid: i64,
    pub track: String,
    pub car: String,
}

#[derive(Debug, Deserialize)]
pub struct InsertScoreRequest {
    pub steamid: i64,
    pub track: String,
    pub car: String,
    pub score: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScoreResponse {
    pub data: i64,
}

#[derive(Debug, Serialize)]
pub struct DefaultResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug)]
pub struct PlaceholderError {}

impl fmt::Display for PlaceholderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Placeholder error")
    }
}

impl Reject for PlaceholderError {}