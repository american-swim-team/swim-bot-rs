pub mod models;
pub mod config;
pub mod routes;
pub mod handlers;
mod errors;

use warp::Filter;
use routes::*; // Import the route functions

// This function will combine all the routes and return them as a single filter
pub fn combined_routes(app_state: models::AppState) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    check_steamid_route(app_state.clone())
        .or(heartbeat_route()).recover(errors::handle_rejection)
}