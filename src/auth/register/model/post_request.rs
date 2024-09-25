use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub username: String,
    pub password: String,
    pub photo: String,
}
