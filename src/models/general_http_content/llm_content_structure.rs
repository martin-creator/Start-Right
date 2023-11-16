use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
/// Message struct sending messages openai
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
/// ChatCompletion struct sending messages openai
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    
}

#[derive(Debug, Deserialize)]
/// ChatResponse struct receiving messages from openai
pub struct APIMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
/// ChatResponse struct receiving messages from openai
pub struct APIChoice {
    pub message: APIMessage,
}

#[derive(Debug, Deserialize)]
/// ChatResponse struct receiving messages from openai
pub struct APIResponse {
    pub choices: Option<Vec<APIChoice>>,
}
