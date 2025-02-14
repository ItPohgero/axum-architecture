use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UpdateUserDto {
    pub username: String,
}
