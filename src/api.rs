use crate::config::Prompt;
use crate::event::Event;
use curl::easy::{Easy, List};
use log::debug;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use std::str::{self, Utf8Error};
use std::sync::mpsc;
use tiktoken_rs::ChatCompletionRequestMessage as TokenChatCompletionRequestMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System => write!(f, "system"),
            Self::User => write!(f, "user"),
            Self::Assistant => write!(f, "assistant"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub content: String,
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ChatMessage {
    #[must_use]
    pub const fn new(content: String, role: Role) -> Self {
        Self {
            content,
            role,
            name: None,
        }
    }
}

impl From<Prompt> for ChatMessage {
    fn from(prompt: Prompt) -> Self {
        Self {
            content: prompt.content,
            role: prompt.role,
            name: None,
        }
    }
}

impl From<ChatMessage> for TokenChatCompletionRequestMessage {
    fn from(val: ChatMessage) -> Self {
        Self {
            role: val.role.to_string(),
            content: Some(val.content),
            name: val.name,
            function_call: None,
        }
    }
}

impl std::fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    messages: Vec<ChatMessage>,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_logprobs: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

impl Default for ChatCompletionRequest {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            model: "gpt-3.5-turbo".to_string(),
            frequency: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_tokens: None,
            n: None,
            presence_penalty: None,
            seed: None,
            stop: None,
            stream: None,
            temperature: None,
            top_p: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    index: i32,
    message: ChatMessage,
    logprobs: Option<()>,
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

impl ChatCompletionRequest {
    fn new(messages: Vec<ChatMessage>, model: String, stream: bool) -> Self {
        Self {
            messages,
            model,
            stream: Some(stream),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    system_fingerprint: String,
    choices: ChatCompletionChoice,
    usage: ChatCompletionUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionDelta {
    content: Option<String>,
    role: Option<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionStreamChoice {
    index: i32,
    delta: ChatCompletionDelta,
    logprobs: Option<()>,
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    id: String,
    object: String,
    created: i64,
    model: String,
    system_fingerprint: String,
    choices: Vec<ChatCompletionStreamChoice>,
}

/// A struct to iterate over a byte stream, yielding `serde_json::Value` objects from SSE data.
#[derive(Debug, Clone)]
pub struct SSEIterator {
    buffer: Vec<u8>,
    counter: usize,
}

impl SSEIterator {
    /// Creates a new `SSEIterator`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buffer: Vec::new(),
            counter: 0,
        }
    }

    /// Appends new bytes to the internal buffer.
    pub fn push(&mut self, bytes: Vec<u8>) {
        self.buffer.extend(bytes);
    }
}

impl Default for SSEIterator {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for SSEIterator {
    type Item = Result<ChatCompletionChunk, Utf8Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let buff = self.buffer.clone();
        let buffer_str = match str::from_utf8(&buff) {
            Ok(bs) => bs,
            Err(e) => return Some(Err(e)),
        };

        debug!("Buffer: {:?}", buffer_str);

        if let Some((pos, _)) = buffer_str.match_indices("\n\n").next() {
            let (event, remaining) = buffer_str.split_at(pos);
            self.buffer = Vec::from(remaining[2..].as_bytes()); // Skip the "\n\n" and keep the rest.
            let event = event.trim_start_matches("data: "); // Remove the `data: ` prefix from the event string.

            match serde_json::from_str::<ChatCompletionChunk>(event) {
                Ok(json) => {
                    self.counter += 1;
                    Some(Ok(json))
                }
                Err(_e) => None,
            }
        } else {
            None // No complete JSON object was found.
        }
    }
}

fn parse_data(data: &[u8], sse_parser: &mut SSEIterator, channel: &mpsc::Sender<Event>) -> usize {
    debug!("Received data: {:?}", data);

    // Iterate through the SSEIterator to get JSON objects.
    for json_result in sse_parser.clone() {
        match json_result {
            Ok(json) => debug!("Parsed JSON object: {:?}", json),
            Err(e) => debug!("Error parsing JSON: {}", e),
        }
    }

    sse_parser.push(data.to_vec());
    match sse_parser.next() {
        Some(Ok(chunk)) => {
            debug!("Received chunk: {:?}", chunk);
            let content = chunk.choices.last().unwrap().delta.content.clone().unwrap();
            let first = sse_parser.counter == 1;
            channel.send(Event::Token(content, first)).unwrap();
        }
        Some(Err(e)) => {
            debug!("Error parsing JSON object: {}", e);
        }
        None => {
            debug!("No complete JSON object found.");
        }
    }

    data.len()
}

pub fn stream_chat_completion(
    requests: Vec<ChatMessage>,
    api_key: &str,
    base_url: &str,
    model: &str,
    channel: mpsc::Sender<Event>,
) {
    let request = ChatCompletionRequest::new(requests, model.to_string(), true); // Placeholder for your actual request initialization
    debug!("Request: {:?}", request);
    let request_payload = serde_json::to_string(&request).unwrap(); // Serializing the request payload
    let mut data = Cursor::new(request_payload.into_bytes());

    let url = format!("{base_url}/chat/completions");

    let mut headers = List::new();
    headers
        .append(&format!("Authorization: Bearer {api_key}"))
        .unwrap();
    headers.append("Content-Type: application/json").unwrap();

    let mut easy = Easy::new();
    easy.post(true).unwrap();
    easy.url(&url).unwrap();
    easy.http_headers(headers).unwrap();
    easy.post_field_size(data.get_ref().len() as u64).unwrap();

    easy.read_function(move |buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();

    let mut sse_parser = SSEIterator::new();

    easy.write_function(move |data| Ok(parse_data(data, &mut sse_parser, &channel)))
        .unwrap();

    easy.perform().unwrap();
}
