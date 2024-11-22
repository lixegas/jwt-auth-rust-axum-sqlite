pub use crate::jwt::Claims;
use axum::async_trait;
use axum::http::{header, request::Parts, StatusCode};
use axum::response::Response;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct Auth(pub String);

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = Response<String>;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.split(" ").nth(1));

        match token {
            Some(token) => match decode::<Claims>(
                token,
                &DecodingKey::from_secret("mykey".as_bytes()),
                &Validation::default(),
            ) {
                Ok(data) => Ok(Auth(data.claims.email)),
                Err(_) => Err(
                    Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body("Invalid token".to_string())
                        .unwrap_or_default(),
                ),
            },
            None => Err(
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("Token missing".to_string())
                    .unwrap_or_default(),
            ),
        }
    }
}
