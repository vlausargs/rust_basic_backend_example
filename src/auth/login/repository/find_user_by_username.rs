use std::io;

use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::base::model::user_model::User;

pub async fn execute(client: &Client, username: &String) -> Result<Vec<User>, io::Error> {
    let query = client
        .prepare("SELECT * FROM \"user\" where username = $1")
        .await
        .unwrap();

    let users = client
        .query(&query, &[username])
        .await
        .expect("Error getting users")
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();
    if users.len() == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "User not found"));
    }
    Ok(users)
}
