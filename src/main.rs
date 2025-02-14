pub mod domain;
pub mod interface;

use axum::{
    routing::{get, post, put, delete},
    Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(interface::controllers::hello_controller::hello))
        .route("/users", post(interface::controllers::user_controller::create_user))
        .route("/users", get(interface::controllers::user_controller::list_users))
        .route("/users/{id}", get(interface::controllers::user_controller::get_user))
        .route("/users/{id}", put(interface::controllers::user_controller::update_user))
        .route("/users/{id}", delete(interface::controllers::user_controller::delete_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}