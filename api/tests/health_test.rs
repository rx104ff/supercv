use axum::{Router, routing::get};
use http::Request;
use hyper::{body::Body, Response};

use std::net::SocketAddr;
use tokio::task;
use reqwest;
use reqwest::StatusCode;

#[tokio::test]
async fn test_health_check() {
    // Start app on a random port in a background task
    let app = Router::new().route("/health", get(|| async { "ok" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = axum::serve(listener, app.into_make_service());

    task::spawn(async move {
        server.await.unwrap();
    });

    // Send a real HTTP request
    let resp = reqwest::get(format!("http://{}/health", addr))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.text().await.unwrap(), "ok");
}
