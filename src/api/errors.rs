use std::convert::From;
use warp::reject::Reject;
use warp::Reply;
use warp::Rejection;
use warp::http::StatusCode;
use poise::serenity_prelude::Error as SerenityError;
use tokio_postgres::Error as PostgresError;
use crate::api::models;

#[derive(Debug)]
pub struct DiscordError(SerenityError);

impl From<SerenityError> for DiscordError {
    fn from(err: SerenityError) -> Self {
        DiscordError(err)
    }
}

impl Reject for DiscordError {}

#[derive(Debug)]
pub struct DatabaseError(PostgresError);

impl From<PostgresError> for DatabaseError {
    fn from(err: PostgresError) -> Self {
        DatabaseError(err)
    }
}

impl Reject for DatabaseError {}

async fn construct_response(code: StatusCode, message: String) -> (models::DefaultResponse, StatusCode) {
    (models::DefaultResponse {
        status: code.as_str().to_string(),
        message: message,
    }, code)
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    // dbg!("Handling rejection: {:?}", &err);
    let mut response = construct_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()).await;

    if err.is_not_found() {
        response = construct_response(StatusCode::NOT_FOUND, "Not Found".to_string()).await;
    } else if let Some(_) = err.find::<DiscordError>() {
        log::error!("Discord error encountered: {:?}", err);
        response = construct_response(StatusCode::INTERNAL_SERVER_ERROR, "Discord error encountered".to_string()).await;
    } else if let Some(e) = err.find::<DatabaseError>() {
        log::error!("Database error encountered: {:?}", e.0);
        response = construct_response(StatusCode::EXPECTATION_FAILED, e.0.to_string()).await;
    } else if let Some(e) = err.find::<warp::reject::MethodNotAllowed>() {
        log::debug!("Method not allowed: {:?}", e);
        response = construct_response(StatusCode::METHOD_NOT_ALLOWED, "Method not allowed".to_string()).await;
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        log::debug!("Bad request: {:?}", e);
        response = construct_response(StatusCode::BAD_REQUEST, "Bad request".to_string()).await;
    } else if let Some(e) = err.find::<warp::reject::UnsupportedMediaType>() {
        log::debug!("Unsupported media type: {:?}", e);
        response = construct_response(StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported media type".to_string()).await;
    } else {
        log::error!("Internal server error encountered: {:?}", err);
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&response.0),
        response.1,
    ))
}
