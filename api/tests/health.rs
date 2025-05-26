use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use tower::Service;
use supercv::interface::routes::create_router;

#[tokio::test]
async fn test_health_check() {
    let mut app = create_router();

    let response = app
        .call(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body.as_ref(), b"{\"status\":\"ok\"}");

}
