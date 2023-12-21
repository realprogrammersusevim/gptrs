use async_openai::types::ChatCompletionRequestSystemMessageArgs;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestUserMessageContent::{Array, Text},
};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};
use textwrap::wrap;

use crate::config::{Prompt, Role};

#[derive(Default, Clone, Debug)]
pub struct ChatHistory {
    pub history: Vec<ChatCompletionRequestMessage>,
    pub current_response: String,
    pub text_width: u16,
}

impl ChatHistory {
    pub fn render_history(&mut self) -> Vec<Line> {
        let mut message_text = vec![];
        for message in self.history.iter() {
            match message {
                ChatCompletionRequestMessage::User(message) => {
                    let text = match &message.content {
                        Some(content) => match content {
                            Text(text) => text.to_owned(),
                            Array(_) => panic!("GPTrs only supports text."),
                        },
                        None => "".to_string(),
                    };
                    let wrapped = wrap(&text, self.text_width as usize); // TODO: Don't hardcode this value
                    for line in wrapped {
                        message_text.push(Line::styled(
                            line.to_string(),
                            Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                ChatCompletionRequestMessage::Assistant(message) => {
                    let text = message.content.clone().unwrap_or("No content".to_string());
                    let wrapped = wrap(&text, self.text_width as usize); // TODO: Don't hardcode this either
                    for line in wrapped {
                        message_text
                            .push(Line::styled(line.to_string(), Style::new().bg(Color::Red)));
                    }
                }
                _ => {}
            }
        }

        message_text
    }

    pub fn push(&mut self, prompt: Prompt) {
        let message = match prompt.role {
            Role::User => ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt.content)
                    .build()
                    .unwrap(),
            ),
            Role::Assistant => ChatCompletionRequestMessage::Assistant(
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content(prompt.content)
                    .build()
                    .unwrap(),
            ),
            Role::System => ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(prompt.content)
                    .build()
                    .unwrap(),
            ),
        };

        self.history.push(message);
    }

    pub fn push_stream(&mut self, text: String, first: bool) {
        self.current_response += &text;
        if !first {
            self.history.pop();
        }
        self.history.push(ChatCompletionRequestMessage::Assistant(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(&self.current_response)
                .build()
                .unwrap(),
        ))
    }

    pub fn len(&mut self) -> usize {
        let mut message_lines = 0;
        for message in self.history.iter() {
            match message {
                ChatCompletionRequestMessage::User(message) => {
                    let text = match &message.content {
                        Some(content) => match content {
                            Text(text) => text.to_owned(),
                            Array(_) => panic!("GPTrs only supports text."),
                        },
                        None => "".to_string(),
                    };
                    let wrapped = wrap(&text, self.text_width as usize); // TODO: Don't hardcode this value
                    message_lines += wrapped.len();
                }
                ChatCompletionRequestMessage::Assistant(message) => {
                    let text = message.content.clone().unwrap_or("No content".to_string());
                    let wrapped = wrap(&text, self.text_width as usize); // TODO: Don't hardcode this either
                    message_lines += wrapped.len();
                }
                _ => {}
            }
        }

        message_lines
    }

    pub fn is_empty(&mut self) -> bool {
        self.history.is_empty()
    }

    pub fn clear_message(&mut self) {
        self.current_response = "".to_string();
    }
}
