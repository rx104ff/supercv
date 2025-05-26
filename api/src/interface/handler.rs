// File: src/interface/handler.rs
use axum::{Json, http::StatusCode};
use serde::Deserialize;
use crate::application::prompt_service::generate_one_shot_prompt;
use crate::infrastructure::openai::send_chat_request;

#[derive(Deserialize)]
pub struct MatchRequest {
    pub resume_latex: String,
    pub job_description: String,
    pub user_note: Option<String>,
}

pub async fn match_resume_handler(
    Json(payload): Json<MatchRequest>
) -> Result<String, (StatusCode, String)> {
    let prompt = generate_one_shot_prompt(
        &payload.resume_latex,
        &payload.job_description,
        payload.user_note.as_deref()
    );

    match send_chat_request(prompt).await {
        Ok(output) => Ok(output),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}
