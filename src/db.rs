use std::env;
// use std::error::Error;

use dotenvy::dotenv;
use libsql::{Builder, Database};

use crate::models::User;

pub async fn get_users() -> Option<Vec<User>> {
    let mut users = Vec::new();
    let db = get_database_connection().await;
    let connection = db.connect().unwrap();
    let mut results = connection.query("SELECT * FROM users", ()).await.unwrap();

    while let Some(row) = results.next().await.unwrap() {
        let user = User {
            first_name: row.get(1).unwrap(),
            last_name: row.get(2).unwrap(),
            username: row.get(3).unwrap(),
            email: row.get(4).unwrap(),
        };
        users.push(user);
    }

    match users.len() > 0 {
        true => Some(users),
        false => None
    }
}

async fn get_database_connection() -> Database {
    dotenv().expect(".env file not found");

    // let db_file = env::var("LOCAL_DATABASE").unwrap();
    let auth_token = env::var("TURSO_AUTH_TOKEN").unwrap();
    let db_url = env::var("TURSO_DATABASE_URL").unwrap();

    let db = Builder::new_remote(db_url, auth_token)
        .build()
        .await
        .unwrap();

    db
}
