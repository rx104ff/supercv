// File: src/interface/routes.rs
use axum::{Router, routing::post};
use crate::interface::handler::match_resume_handler;

pub fn create_router() -> Router {
    Router::new()
        .route("/match", post(match_resume_handler))
}
