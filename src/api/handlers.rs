use super::models::AppState;
use super::models;
use poise::serenity_prelude as serenity;

pub async fn check_steamid(data: models::CheckSteamid, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Checking steamid: {:?}", &data);

    match state.database.query_one(
        "SELECT discordid FROM steamids WHERE steamid = $1",
        &[&data.steamid],
    ).await {
        Ok(row) => {
            let discordid: i64 = row.get(0);
            let user_roles: Vec<serenity::model::prelude::RoleId> = match state.http.get_member(state.config.discord.guild.into(), serenity::UserId::from(discordid as u64)).await {
                Ok(member) => member.roles.to_vec(),
                Err(e) => {
                    log::error!("Failed to fetch member roles: {}", e);
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
            log::error!("Failed to fetch discordid: {}", e);
            return Ok(warp::reply::json(&models::DefaultResponse {
                status: "UNAUTHORIZED".to_string(),
                message: "Not authorized".to_string(),
            }))
        }
    }
}

pub async fn fetch_cutup_score(data: models::ScoreRequest, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Cutup highscore request: {:?}", &data);
    
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
            log::error!("Failed to fetch highscore: {}", e);
            return Ok(warp::reply::json(&models::ScoreResponse {
                data: 0,
            }))
        }
    }
}

pub async fn insert_cutup_score(data: models::InsertScoreRequest, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Cutup highscore insert: {:?}", &data);

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
            log::error!("Failed to insert highscore: {}", e);
            return Ok(warp::reply::json(&models::DefaultResponse {
                status: "ERROR".to_string(),
                message: "Failed to insert".to_string(),
            }))
        }
    }
}

pub async fn update_driver_stats(data: models::UpdateDriverStatsRequest, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Driver stats update: {:?}", &data);

    let result = state.database.execute(
        "INSERT INTO driver_stats (steamid, track, collisions, distance, avgspeed, total_time)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (steamid, track)
         DO UPDATE SET collisions = driver_stats.collisions + EXCLUDED.collisions,
                       distance = driver_stats.distance + EXCLUDED.distance,
                       avgspeed = (driver_stats.avgspeed + EXCLUDED.avgspeed) / 2,
                       total_time = driver_stats.total_time + EXCLUDED.total_time
         WHERE driver_stats.steamid = EXCLUDED.steamid AND
               driver_stats.track = EXCLUDED.track",
        &[&data.steamid, &data.track, &data.collisions, &data.distance, &data.avgspeed, &data.time],
    ).await;

    match result {
        Ok(_) => Ok(warp::reply::json(&models::DefaultResponse {
            status: "OK".to_string(),
            message: "Driver stats updated".to_string(),
        })),
        Err(e) => {
            log::error!("Failed to update driver stats: {}", e);
            Ok(warp::reply::json(&models::DefaultResponse {
                status: "ERROR".to_string(),
                message: "Failed to update driver stats".to_string(),
            }))
        }
    }
}

// pub async fn fetch_lap_time(data: models::ScoreRequest, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
//     dbg!("Lap time request: {:?}", &data);
    
//     match state.database.query_one(
//         "SELECT score FROM lap WHERE steamid = $1 AND map = $2 AND car = $3",
//         &[&data.steamid, &data.track, &data.car],
//     ).await {
//         Ok(row) => {
//             let score: i64 = row.get(0);
//             return Ok(warp::reply::json(&models::ScoreResponse {
//                 data: score,
//             }))
//         },
//         Err(e) => {
//             dbg!(e);
//             return Ok(warp::reply::json(&models::ScoreResponse {
//                 data: 0,
//             }))
//         }
//     }
// }

pub async fn heartbeat() -> Result<impl warp::Reply, warp::Rejection> {
    let response = models::DefaultResponse {
        status: "OK".to_string(),
        message: "Alive".to_string(),
    };
    Ok(warp::reply::json(&response))
}