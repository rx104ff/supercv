use crate::models::message_payload::{Message, Role};

pub fn generate_one_shot_prompt(
    full_resume_latex: &str,
    job_desc_raw: &str,
    user_note: Option<&str>,
) -> Vec<Message> {
    let mut instructions = vec![
        "You are a professional technical resume editor.",
        "The user will provide a LaTeX resume and a job description.",
        "Your task is to rewrite the LaTeX content to tailor it for the job description.",
        "Keep the formatting valid LaTeX.",
        "Only modify sections that you think can be improved for the job fit.",
        "Leave all unrelated sections unchanged.",
        "Output the full rewritten LaTeX file.",
    ];

    if let Some(note) = user_note {
        instructions.push(note);
    }

    let full_prompt = format!(
        "{}\n\n--- RESUME (LaTeX) ---\n{}\n\n--- JOB DESCRIPTION ---\n{}",
        instructions.join("\n"),
        full_resume_latex,
        job_desc_raw
    );

    vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant specialized in LaTeX resume tailoring.".to_string(),
        },
        Message {
            role: Role::User,
            content: full_prompt,
        }
    ]
}
