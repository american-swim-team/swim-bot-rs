use std::convert::From;
use warp::reject::Reject;
use warp::Reply;
use warp::Rejection;
use warp::http::StatusCode;
use serenity::Error as SerenityError;
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

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<DiscordError>() {
        println!("Discord error encountered: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Discord Error";
    } else if let Some(_) = err.find::<DatabaseError>() {
        println!("Database error encountered: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Database Error";
    } else {
        // Unhandled rejections are turned into 500 Internal Server Error responses.
        println!("Error encountered: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let error_response = models::DefaultResponse {
        status: code.as_str().to_string(),
        message: message.to_string(),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&error_response),
        code,
    ))
}
