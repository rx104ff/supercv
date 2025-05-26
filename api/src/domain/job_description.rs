#[derive(Debug)]
pub struct JobDescription {
    pub title: String,
    pub company: String,
    pub required_skills: Vec<String>,
    pub preferred_qualities: Vec<String>,
}

impl JobDescription {
    pub fn from_raw(text: &str) -> Self {
        // For now, just stub or do regex-based extraction
        // Later we can use ChatGPT to parse this as well
        Self {
            title: "Software Engineer".to_string(),
            company: "Acme Corp".to_string(),
            required_skills: vec!["Rust".to_string(), "AWS".to_string()],
            preferred_qualities: vec!["Open source contributions".to_string()],
        }
    }
}
