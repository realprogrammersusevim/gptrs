use crate::config::Prompt;
use crate::event::Event;
use curl::easy::{Easy, List};
use log::debug;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::io::{Cursor, Read};
use std::rc::Rc;
use std::str;
use std::sync::mpsc;
use std::sync::Mutex;
use std::time::Duration;
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
    frequency: Option<f32>,
    logit_bias: Option<f32>,
    logprobs: Option<bool>,
    top_logprobs: Option<i8>,
    max_tokens: Option<u16>,
    n: Option<u8>,
    presence_penalty: Option<f32>,
    seed: Option<i32>,
    stop: Option<String>,
    stream: Option<bool>,
    temperature: Option<f32>,
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

/// A single Server-Sent Event.
#[derive(Debug)]
pub struct StreamEvent {
    /// Corresponds to the `id` field.
    pub id: Option<String>,
    /// Corresponds to the `event` field.
    pub event_type: Option<String>,
    /// All `data` fields concatenated by newlines.
    pub data: String,
}

/// Possible results from parsing a single event-stream line.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseResult {
    /// Line parsed successfully, but the event is not complete yet.
    Next,
    /// The event is complete now. Pass a new (empty) event for the next call.
    Dispatch,
    /// Set retry time.
    SetRetry(Duration),
}

pub fn parse_event_line(line: &str, event: &mut StreamEvent) -> ParseResult {
    let line = line.trim_end_matches(|c| c == '\r' || c == '\n');
    if line.is_empty() {
        debug!("Line is empty");
        ParseResult::Dispatch
    } else {
        debug!("Line is not empty");
        let (field, value) = line.find(':').map_or((line, ""), |pos| {
            let (f, v) = line.split_at(pos);
            // Strip : and an optional space.
            let v = &v[1..];
            let v = if v.starts_with(' ') { &v[1..] } else { v };
            (f, v)
        });

        match field {
            "event" => {
                event.event_type = Some(value.to_string());
            }
            "data" => {
                event.data.push_str(value);
                event.data.push('\n');
            }
            "id" => {
                event.id = Some(value.to_string());
            }
            "retry" => {
                if let Ok(retry) = value.parse::<u64>() {
                    return ParseResult::SetRetry(Duration::from_millis(retry));
                }
            }
            _ => (), // ignored
        }

        ParseResult::Next
    }
}

// pub fn stream_chat_completion(
//     requests: Vec<ChatMessage>,
//     api_key: &str,
//     base_url: &str,
//     model: &str,
//     channel: mpsc::Sender<Event>,
// ) {
//     let request = ChatCompletionRequest::new(requests, model.to_string(), true);
//     let request_payload = serde_json::to_string(&request).unwrap();
//     let mut data = Cursor::new(request_payload.into_bytes());
//
//     let url = format!("{base_url}/chat/completions");
//
//     let mut headers = List::new();
//     headers
//         .append(format!("Authorization: Bearer {api_key}").as_str())
//         .unwrap();
//     headers.append("Content-Type: application/json").unwrap();
//
//     let mut easy = Easy::new();
//     easy.post(true).unwrap();
//     easy.url(&url).unwrap();
//     easy.http_headers(headers).unwrap();
//     easy.post_field_size(data.get_ref().len() as u64).unwrap();
//
//     easy.read_function(move |buf| Ok(data.read(buf).unwrap_or(0)))
//         .unwrap();
//
//     debug!("All good so far. About to set up the write_function");
//
//     let mut accumulator = Vec::new();
//     let mut current_event = StreamEvent {
//         id: None,
//         event_type: None,
//         data: String::new(),
//     };
//     let mut first = true;
//
// Your original `stream_chat_completion` function, adjusted for SSE parsing
pub fn stream_chat_completion(
    requests: Vec<ChatMessage>,
    api_key: &str,
    base_url: &str,
    model: &str,
    channel: mpsc::Sender<Event>,
) {
    let request = ChatCompletionRequest::new(requests, model.to_string(), true); // Placeholder for your actual request initialization
    let request_payload = serde_json::to_string(&request).unwrap(); // Serializing the request payload
    let mut data = Cursor::new(request_payload.into_bytes());

    let url = format!("{}/chat/completions", base_url);

    let mut headers = List::new();
    headers
        .append(&format!("Authorization: Bearer {}", api_key))
        .unwrap();
    headers.append("Content-Type: application/json").unwrap();

    let mut easy = Easy::new();
    easy.post(true).unwrap();
    easy.url(&url).unwrap();
    easy.http_headers(headers).unwrap();
    easy.post_field_size(data.get_ref().len() as u64).unwrap();

    easy.read_function(move |buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();

    let mut accumulator = Vec::new();
    let current_event = StreamEvent {
        id: None,
        event_type: None,
        data: String::new(),
    }; // Placeholder for your actual event initialization
    let mut first = true;

    easy.write_function(move |data| {
        accumulator.extend_from_slice(data);

        while let Some(pos) = accumulator.iter().position(|&x| x == b'\n') {
            let next = pos + 1;
            if next < accumulator.len() && accumulator[next] == b'\n' {
                // We've found a double newline, indicating the end of an event
                let event_data = &accumulator[..next - 1]; // Exclude the first newline
                let event_str = str::from_utf8(event_data).unwrap_or_default();

                // Process the SSE event string (`event_str`) here
                // This is where you should parse the `event_str` to handle different fields like "event:" and "data:"
                // For simplification, let's assume `event_str` directly contains the data you're interested in and dispatch it
                if let Ok(parsed_chunk) = serde_json::from_str::<ChatCompletionChunk>(event_str) {
                    let event = Event::Token(
                        parsed_chunk
                            .choices
                            .first()
                            .unwrap()
                            .delta
                            .content
                            .clone()
                            .unwrap(),
                        first,
                    );
                    channel.send(event).unwrap();
                    first = false; // Update the first flag after sending the first event
                }

                // Clear processed event data from the accumulator
                accumulator.drain(..=next);
            } else {
                // break; // If we haven't found a complete event, break
            }
        }

        Ok(data.len())
    })
    .unwrap();

    easy.perform().unwrap();
}

//     easy.write_function(move |data| {
//         accumulator.extend_from_slice(data);
//
//         let mut deletion_index = 0;
//
//         // Process each line as it's completed
//         for (index, &item) in accumulator.iter().enumerate() {
//             debug!("{}: {}", index, item.to_string());
//             if item == b'\n' || item == b'\r' {
//                 debug!("Found the end of a line.");
//                 // Extract the line from the accumulator and process it
//                 let line = str::from_utf8(&accumulator[..index]).unwrap_or_default();
//                 // Your line processing logic here
//                 deletion_index = index + 1;
//
//                 match parse_event_line(line, &mut current_event) {
//                     ParseResult::Dispatch => {
//                         debug!("At ParseResult::Dispatch");
//                         // Here, you’d handle the completed `current_event`
//                         // For example, parse `current_event.data` as JSON if expected, then send through channel
//
//                         // If the event data is expected to be a JSON string resembling ChatCompletionChunk:
//                         if let Ok(parsed_chunk) =
//                             serde_json::from_str::<ChatCompletionChunk>(&current_event.data)
//                         {
//                             debug!("Parsed chunk: {:?}", parsed_chunk);
//                             let event = Event::Token(
//                                 parsed_chunk
//                                     .choices
//                                     .first()
//                                     .unwrap()
//                                     .delta
//                                     .content
//                                     .clone()
//                                     .unwrap(),
//                                 first,
//                             );
//                             channel.send(event).unwrap();
//                             first = false; // Update the first flag appropriately after sending first event
//                         } else {
//                             debug!("Failed to parse chunk");
//                         }
//
//                         // Prepare for the next event
//                         current_event = StreamEvent {
//                             id: None,
//                             event_type: None,
//                             data: String::new(),
//                         };
//                     }
//                     ParseResult::Next => {
//                         debug!("At ParseResult::Next");
//                         // Handle next
//                     }
//                     ParseResult::SetRetry(retry_duration) => {
//                         debug!("At ParseResult::SetRetry");
//                         // Optionally handle retry logic here, for example:
//                         debug!("Set retry duration to: {:?}", retry_duration);
//                     }
//                 }
//             }
//         }
//
//         // Remove processed bytes from accumulator
//         accumulator.drain(..deletion_index);
//
//         debug!("All done with the write_function. About to return.");
//
//         Ok(data.len())
//     })
//     .unwrap();
//
//     easy.perform().unwrap();
// }
