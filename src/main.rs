mod db;
mod public;
mod auth;
mod jwt;
mod middleware;
mod models;

use axum::{routing::{get, post}, Router, Server};
use std::sync::Arc;
use std::net::TcpListener;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let pool = db::initialize_database().await.expect("Failed to initialize database");

    
    let routes = Router::new()
        .route("/public-view", get(public::public_view_handler))
        .route("/register", post(auth::register_handler))
        .route("/login", post(auth::login_handler))
        .route("/secret-view", get(auth::secret_view_handler))
        .with_state(Arc::new(pool));

    
    let tcp_listener = TcpListener::bind("127.0.0.1:2323")
        .expect("Address should be free and valid");

    Server::from_tcp(tcp_listener)
        .unwrap()
        .serve(routes.into_make_service())
        .await
        .expect("Error serving application");
}
