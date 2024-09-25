use std::io::{Error, ErrorKind};

use deadpool_postgres::Client;

use crate::base::model::user_model::User;

pub async fn execute(client: &Client, user: &User) -> Result<User, Error> {
    let query = client
        .prepare(
            r#"INSERT INTO "user" 
            (
                id
                , "name"
                , username
                , email
                , phone
                , password
                , photo
                , created_at
                , updated_at
            ) 
            values
            (
                $1
                , $2
                , $3
                , $4
                , $5
                , $6
                , $7
                , now()
                , now()
            ) 
            returning id, "name", username, email, phone, photo, created_at, updated_at
            "#,
        )
        .await
        .unwrap();

    let res = client
        .query(
            &query,
            &[
                &(user.id),
                &(user.name),
                &(user.username),
                &(user.email),
                &(user.phone),
                &(user.password),
                &(user.photo),
            ],
        )
        .await;

    return match res {
        Ok(rows) => rows
            .iter()
            .map(|row| User::from(row))
            .collect::<Vec<User>>()
            .pop()
            .ok_or(Error::new(ErrorKind::Other, "Error creating user")),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    };
}
