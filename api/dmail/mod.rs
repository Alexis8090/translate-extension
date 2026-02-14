use axum::Router;
use user::build_user_routes;
use crate::AppState;

pub mod user;

pub fn build_dmail_routes() ->  Router<AppState> {
       Router::new().nest("/dmail", build_user_routes())
}
