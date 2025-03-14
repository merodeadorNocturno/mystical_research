use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateContentResponse {
    pub candidates: Vec<Candidate>,
}

impl GenerateContentResponse {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        Self { candidates }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub api_endpoint: String,
    pub request_body: GenerateContentRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    pub response_body: GenerateContentResponse,
    pub response_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogStructure {
    pub title: String,
    pub table_of_contents: Vec<String>,
    pub summary: String,
    pub content: String,
    pub keywords: Vec<String>,
}
