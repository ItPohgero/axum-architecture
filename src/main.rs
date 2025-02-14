pub mod interface;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(interface::controllers::hello_controller::hello))
        // `POST /users` goes to `create_user`
        .route(
            "/users",
            post(interface::controllers::user_controller::create_user),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
