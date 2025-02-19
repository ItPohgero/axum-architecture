pub mod domain;
pub mod delivery;
pub mod configs;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let env = configs::environment::main();

    let app = Router::new()
        .route("/", get(delivery::controllers::hello_controller::hello))
        .route("/users", post(delivery::controllers::user_controller::create_user))
        .route("/users", get(delivery::controllers::user_controller::list_users))
        .route("/users/{id}", get(delivery::controllers::user_controller::get_user))
        .route("/users/{id}", put(delivery::controllers::user_controller::update_user))
        .route("/users/{id}", delete(delivery::controllers::user_controller::delete_user));

    let listener: TcpListener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", env.port)
    ).await.unwrap();

    println!("Server running on 0.0.0.0:{}", env.port);
    axum::serve(listener, app).await.unwrap();
}