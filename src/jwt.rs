pub use crate::models::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};


pub fn generate_jwt(email: &str) -> String {
    let claims = Claims {
        email: email.to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("mykey".as_bytes()),
    )
    .unwrap()
}
