// File: src/interface/handler.rs
use axum::{Json, http::StatusCode, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::application::prompt_service::generate_one_shot_prompt;
use std::sync::Arc;
use crate::application::gpt_api::GptApi;

#[derive(Deserialize)]
pub struct MatchRequest {
    pub resume_latex: String,
    pub job_description: String,
    pub user_note: Option<String>,
}

pub async fn match_resume_handler(
    State(gpt): State<Arc<dyn GptApi>>,
    Json(payload): Json<MatchRequest>
) -> Result<String, (StatusCode, String)> {
    let messages = generate_one_shot_prompt(
        &payload.resume_latex,
        &payload.job_description,
        payload.user_note.as_deref()
    );

    match gpt.send_chat_request(messages).await {
        Ok(resp) => Ok(resp),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
