pub mod domain;
pub mod delivery;
pub mod configs;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let env = configs::environment::main();

    let app = delivery::routes::create_router();

    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", env.port)
    ).await.unwrap();

    println!("Server running on 0.0.0.0:{}", env.port);
    axum::serve(listener, app).await.unwrap();
}