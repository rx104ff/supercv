use axum::{Router, routing::post};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::task;
use supercv::application::gpt_api::GptApi; // adjust path
use supercv::interface::{routes::create_router}; // you need to expose this in lib.rs
use supercv::test_utils::mocks::MockGptApi;
use serde_json::json;

#[tokio::test]
async fn test_match_with_mocked_gpt() {
    // Prepare mock GPT implementation
    let mock_gpt = Arc::new(MockGptApi::new_with_response("mocked latex response".to_string()));

    // Build router with mocked state
    let app = create_router(mock_gpt);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    task::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    // Perform POST request
    let client = reqwest::Client::new();
    let payload = json!({
        "resume_latex": "sample resume",
        "job_description": "sample job",
        "user_note": "note"
    });

    let res = client
        .post(format!("http://{}/match", addr))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "mocked latex response");
}
