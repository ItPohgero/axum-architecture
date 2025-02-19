mod user;
mod hello;
use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .merge(hello::hello_routes())
        .merge(user::user_routes())
}