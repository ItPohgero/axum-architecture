use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use crate::domain::{
    models::user::User,
    dto::user_dto::{CreateUserDto, UpdateUserDto},
};

// In-memory storage (in real app, this would be in a repository)
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static USERS: Lazy<Mutex<HashMap<u64, User>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub async fn create_user(
    Json(payload): Json<CreateUserDto>
) -> (StatusCode, Json<User>) {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let user = User {
        id,
        username: payload.username,
    };
    
    USERS.lock().unwrap().insert(id, user.clone());
    (StatusCode::CREATED, Json(user))
}

pub async fn get_user(
    Path(id): Path<u64>
) -> Result<Json<User>, StatusCode> {
    let users = USERS.lock().unwrap();
    
    users
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn update_user(
    Path(id): Path<u64>,
    Json(payload): Json<UpdateUserDto>,
) -> Result<Json<User>, StatusCode> {
    let mut users = USERS.lock().unwrap();
    
    if let Some(user) = users.get_mut(&id) {
        user.username = payload.username;
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_user(
    Path(id): Path<u64>
) -> StatusCode {
    let mut users = USERS.lock().unwrap();
    
    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_users() -> Json<Vec<User>> {
    let users = USERS.lock().unwrap();
    Json(users.values().cloned().collect())
}