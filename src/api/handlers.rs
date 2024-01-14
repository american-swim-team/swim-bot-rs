use super::models::AppState;
use super::models;
use poise::serenity_prelude as serenity;

pub async fn check_steamid(data: models::CheckSteamid, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    dbg!("Checking steamid {} for roles {}", &data.steamid, &data.roles);

    match state.database.query_one(
        "SELECT discordid FROM steamids WHERE steamid = $1",
        &[&data.steamid],
    ).await {
        Ok(row) => {
            let discordid: i64 = row.get(0);
            let user_roles: Vec<serenity::model::prelude::RoleId> = match state.discord.get_member(state.config.discord.guild, discordid as u64).await {
                Ok(member) => member.roles,
                Err(e) => {
                    dbg!(e);
                    return Ok(warp::reply::json(&models::DefaultResponse {
                        status: "UNAUTHORIZED".to_string(),
                        message: "Not authorized".to_string(),
                    }))
                }
            };
            if data.roles.iter().any(|role| user_roles.contains(&poise::serenity_prelude::model::id::RoleId::from(*role as u64))) {
                return Ok(warp::reply::json(&models::DefaultResponse {
                    status: "OK".to_string(),
                    message: "Authorized".to_string(),
                }))
            } else {
                return Ok(warp::reply::json(&models::DefaultResponse {
                    status: "UNAUTHORIZED".to_string(),
                    message: "Not authorized".to_string(),
                }))
            }
        },
        Err(e) => {
            dbg!(e);
            return Ok(warp::reply::json(&models::DefaultResponse {
                status: "UNAUTHORIZED".to_string(),
                message: "Not authorized".to_string(),
            }))
        }
    }
}

pub async fn fetch_cutup_score(data: models::ScoreRequest, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    dbg!("Cutup highscore request: {:?}", &data);
    
    match state.database.query_one(
        "SELECT score FROM cutup WHERE steamid = $1 AND track = $2 ORDER BY score DESC LIMIT 1",
        &[&data.steamid, &data.track],
    ).await {
        Ok(row) => {
            let score: i64 = row.get(0);
            return Ok(warp::reply::json(&models::ScoreResponse {
                data: score,
            }))
        },
        Err(e) => {
            dbg!(e);
            return Ok(warp::reply::json(&models::ScoreResponse {
                data: 0,
            }))
        }
    }
}

pub async fn insert_cutup_score(data: models::InsertScoreRequest, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    dbg!("Cutup highscore insert: {:?}", &data);

    if &data.score > &9999999 {
        return Ok(warp::reply::json(&models::DefaultResponse {
            status: "ERROR".to_string(),
            message: "Score too high".to_string(),
        }))
    }
    
    match state.database.execute(
        "INSERT INTO cutup (steamid, track, car, score)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (steamid, track, car)
        DO UPDATE SET score = EXCLUDED.score
        WHERE EXCLUDED.score > cutup.score;",
        &[&data.steamid, &data.track, &data.car, &data.score],
    ).await {
        Ok(_) => {
            return Ok(warp::reply::json(&models::DefaultResponse {
                status: "OK".to_string(),
                message: "Inserted".to_string(),
            }))
        },
        Err(e) => {
            dbg!(e);
            return Ok(warp::reply::json(&models::DefaultResponse {
                status: "ERROR".to_string(),
                message: "Failed to insert".to_string(),
            }))
        }
    }
}

pub async fn fetch_lap_time(data: models::ScoreRequest, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    dbg!("Lap time request: {:?}", &data);
    
    match state.database.query_one(
        "SELECT score FROM lap WHERE steamid = $1 AND map = $2 AND car = $3",
        &[&data.steamid, &data.track, &data.car],
    ).await {
        Ok(row) => {
            let score: i64 = row.get(0);
            return Ok(warp::reply::json(&models::ScoreResponse {
                data: score,
            }))
        },
        Err(e) => {
            dbg!(e);
            return Ok(warp::reply::json(&models::ScoreResponse {
                data: 0,
            }))
        }
    }
}

pub async fn heartbeat() -> Result<impl warp::Reply, warp::Rejection> {
    let response = models::DefaultResponse {
        status: "OK".to_string(),
        message: "Alive".to_string(),
    };
    Ok(warp::reply::json(&response))
}