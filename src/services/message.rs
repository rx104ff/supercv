use crate::models::message_payload::{Message, Role};

pub fn build_resume_update_prompt(section: &str, content: &str) -> Vec<Message> {
    vec![
        Message {
            role: Role::System,
            content: "You are a professional resume editor with knowledge of LaTeX syntax.".into(),
        },
        Message {
            role: Role::User,
            content: format!("Please help rewrite the following LaTeX resume section `{}` to sound more professional:\n\n{}", section, content),
        },
    ]
}
