use warp::Filter;
use super::models::AppState;
use super::handlers;

use super::models;

pub fn check_steamid_route(state: AppState) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    println!("check_steamid_route");
    warp::path!("checksteamid")
        .and(warp::post())
        .and(warp::body::json::<models::CheckSteamid>())
        .and(with_state(state))
        .and_then(handlers::check_steamid)
}

pub fn heartbeat_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("heartbeat")
        .and(warp::get())
        .and_then(handlers::heartbeat)
}

fn with_state(state: AppState) -> impl Filter<Extract = (AppState,), Error = warp::Rejection> + Clone {
    warp::any()
        .and_then(move || {
            let cloned_state = state.clone();
            async move {
                // This is a contrived way to adjust the error type.
                // We immediately produce a rejection...
                let _ = warp::reject::custom(super::models::PlaceholderError {});
                // ...and then immediately override it with the desired state.
                // This ensures that the error type is `Rejection`.
                Ok::<AppState, warp::Rejection>(cloned_state)
            }
        })
}