// File: src/interface/routes.rs
use axum::{Router, routing::{post, get}};
use crate::interface::handler::match_resume_handler;
use crate::interface::handler::health_check;

pub fn create_router() -> Router {
    Router::new()
        .route("/match", post(match_resume_handler))
        .route("/health", get(health_check))
}
