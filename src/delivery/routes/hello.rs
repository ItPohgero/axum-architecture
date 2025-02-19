use axum::{
    routing::get,
    Router,
};
use crate::delivery::controllers::hello_controller;

pub fn hello_routes() -> Router {
    Router::new()
        .route("/", get(hello_controller::hello))
}