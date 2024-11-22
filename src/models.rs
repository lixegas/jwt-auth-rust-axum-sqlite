use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct UserRegistration {
    pub email: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub exp: i64,
}
