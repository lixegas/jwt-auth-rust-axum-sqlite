use axum::http::StatusCode;
use axum::response::Response;
use serde_json::json;

/// Public route handler
pub async fn public_view_handler() -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .body(
            json!({
                "success": true,
                "message": "This data is visible to everyone"
            })
            .to_string(),
        )
        .unwrap_or_default()
}
