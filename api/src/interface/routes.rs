// File: src/interface/routes.rs
use axum::{Router, routing::{post, get}};
use crate::interface::handler::match_resume_handler;
use crate::interface::handler::health_check;
use std::sync::Arc;
use crate::application::gpt_api::GptApi;

pub fn create_router(state: Arc<dyn GptApi>) -> Router {
    Router::new()
        .route("/match", post(crate::interface::handler::match_resume_handler))
        .route("/health", get(crate::interface::handler::health_check))
        .with_state(state)
}
