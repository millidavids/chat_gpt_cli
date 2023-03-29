use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

const DEFAULT_MESSAGE_STR: &str = "Please introduce yourself, ChatGPT.";
const DEFAULT_MODEL: &str = "gpt-3.5-turbo";

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Role {
    #[default]
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
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
        if let Some(mut dir_buf) = dirs::home_dir() {
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
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unable to find home dir.",
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
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
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
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
