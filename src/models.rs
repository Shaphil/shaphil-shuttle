use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(firstname: String, lastname: String, username: String, email: String) -> Self {
        Self {
            first_name: firstname,
            last_name: lastname,
            username,
            email,
        }
    }
}