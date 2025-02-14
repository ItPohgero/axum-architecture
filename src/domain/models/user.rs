use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
}
