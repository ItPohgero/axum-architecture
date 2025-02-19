use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::delivery::controllers::user_controller;

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", post(user_controller::create_user))
        .route("/users", get(user_controller::list_users))
        .route("/users/{id}", get(user_controller::get_user))
        .route("/users/{id}", put(user_controller::update_user))
        .route("/users/{id}", delete(user_controller::delete_user))
}
