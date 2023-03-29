use std::{fs::File, io::{Read, Write}};

use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

const DEFAULT_MESSAGE_STR: &str = "Please introduce yourself, ChatGPT.";
const DEFAULT_MODEL: &str = "gpt-3.5-turbo";

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    #[default]
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            role: Role::default(),
            content: DEFAULT_MESSAGE_STR.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Convo {
    pub name: String,
    pub messages: Vec<Message>,
}

impl Convo {
    pub fn fetch_or_create(name: &String) -> Result<Self, std::io::Error> {
        let mut dir_buf = crate::util::assemble_convo_path()?;
        dir_buf.push(name);
        if let Ok(mut file) = File::open(dir_buf.as_path()) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let convo: Convo = serde_json::from_str(&contents[..])?;
            Ok(convo)
        } else {
            Ok(Convo {
                name: name.clone(),
                messages: vec![],
            })
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut dir_buf = crate::util::assemble_convo_path()?;
        dir_buf.push(&self.name);
        let mut file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(dir_buf.as_path())?;
        write!(file, "{}", serde_json::to_string(&self)?)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequestBody {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl Default for ChatRequestBody {
    fn default() -> Self {
        Self {
            model: DEFAULT_MODEL.into(),
            messages: vec![Message::default()],
            temperature: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

impl ChatResponse {
    pub fn from_api(json: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(std::env::var("OPENAI_API_KEY").unwrap())
            .body(json)
            .send()?;

        Ok(serde_json::from_str(&res.text()?[..])?)
    }
}

#[cfg(test)]
mod tests {
    use super::{ChatRequestBody, DEFAULT_MESSAGE_STR, DEFAULT_MODEL};

    #[test]
    fn chat_request_body() {
        let crb = ChatRequestBody::default();
        let crb_json = serde_json::to_string(&crb).unwrap();
        assert_eq!(crb_json, format!("{{\"model\":\"{DEFAULT_MODEL}\",\"messages\":[{{\"role\":\"User\",\"content\":\"{DEFAULT_MESSAGE_STR}\"}}]}}"));
    }
}
