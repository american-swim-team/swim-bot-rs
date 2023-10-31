use super::models::AppState;
use super::models;
use super::errors::*; 
use serenity::model::id::RoleId;

pub async fn check_steamid(data: models::CheckSteamid, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    dbg!("Checking steamid: {}", data.steamid);
    let discordid: u64 = state.database.fetch_discordid(data.steamid).await.map_err(DatabaseError::from)?.into();
    let user_roles = state.discord.get_member(state.config.discord.guild, discordid).await.map_err(DiscordError::from)?.roles;

    let authorized = data.roles.iter().any(|role| user_roles.contains(&RoleId::from(*role as u64)));

    if authorized {
        dbg!("Authorized");
        Ok(warp::reply::json(&models::DefaultResponse {
            status: "OK".to_string(),
            message: "Authorized".to_string(),
        }))
    } else {
        dbg!("Not authorized");
        Ok(warp::reply::json(&models::DefaultResponse {
            status: "UNAUTHORIZED".to_string(),
            message: "Not authorized".to_string(),
        }))
    }
}

pub async fn heartbeat() -> Result<impl warp::Reply, warp::Rejection> {
    let response = models::DefaultResponse {
        status: "OK".to_string(),
        message: "Alive".to_string(),
    };
    Ok(warp::reply::json(&response))
}