pub mod domain;
pub mod delivery;
pub mod configs;
pub mod helpers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut env = configs::environment::main();

    if tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port)).await.is_err() {
        if let Some(new_port) = helpers::available_port::find(env.port).await {
            println!("Port {} is in use, switching to port {}", env.port, new_port);
            env.port = new_port;
        } else {
            eprintln!("No available ports found");
            std::process::exit(1);
        }
    }

    let app = delivery::routes::create_router();
    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", env.port)
    ).await.unwrap();

    println!("Server running on 0.0.0.0:{}", env.port);
    axum::serve(listener, app).await.unwrap();
}