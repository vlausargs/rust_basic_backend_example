use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::auth::register::model::post_request::RegisterUserRequest;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "user")]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub password: Option<String>,

    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<&Row> for User {
    fn from(row: &Row) -> User {
        User {
            id: row.get("id"),
            name: row.get("name"),
            username: row.get("username"),
            email: row.get("email"),
            phone: row.try_get("phone").ok(),
            photo: row.try_get("photo").ok(),
            password: row.try_get("password").ok(),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl From<&RegisterUserRequest> for User {
    fn from(user: &RegisterUserRequest) -> User {
        let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

        User {
            id: Some(Uuid::new_v4()),
            name: user.name.clone(),
            email: user.email.clone(),
            username: user.username.clone(),
            password: Some(hashed_password.clone()),
            phone: Some(user.phone.clone()),
            photo: Some(user.photo.clone()),
            created_at: None,
            updated_at: None,
        }
    }
}
