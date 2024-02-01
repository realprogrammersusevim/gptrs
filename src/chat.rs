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
pub struct History {
    pub history: Vec<ChatCompletionRequestMessage>,
    pub current_response: String,
    pub text_width: u16,
    pub text_lines: usize,
}

impl History {
    /// # Panics
    ///
    /// Will panic if the response somehow contains something other than text
    pub fn render_history(&mut self) -> Vec<Line> {
        let mut message_text = vec![];
        for message in &self.history {
            match message {
                ChatCompletionRequestMessage::User(message) => {
                    let text = message
                        .content
                        .as_ref()
                        .map_or_else(String::new, |content| match content {
                            Text(text) => text.clone(),
                            Array(_) => panic!("GPTrs only supports text."),
                        });
                    let wrapped = wrap(&text, self.text_width as usize);
                    for line in wrapped {
                        message_text.push(Line::styled(
                            line.to_string(),
                            Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                ChatCompletionRequestMessage::Assistant(message) => {
                    let text = message
                        .content
                        .clone()
                        .unwrap_or_else(|| "No content".to_string());
                    let wrapped = wrap(&text, self.text_width as usize);
                    for line in wrapped {
                        message_text
                            .push(Line::styled(line.to_string(), Style::new().bg(Color::Red)));
                    }
                }
                _ => {}
            }
        }

        self.text_lines = message_text.len();

        message_text
    }

    fn prompt_to_message(prompt: Prompt) -> ChatCompletionRequestMessage {
        match prompt.role {
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
        }
    }

    /// # Panics
    ///
    /// Will panic if the ``ChatCompletionRequestMessage`` cannot be created with the ``Prompt``
    /// content
    pub fn push(&mut self, prompt: Prompt) {
        self.history.push(Self::prompt_to_message(prompt));
    }

    pub fn extend(&mut self, prompts: Vec<Prompt>) {
        let messages = prompts.into_iter().map(Self::prompt_to_message);
        self.history.extend(messages);
    }

    /// # Panics
    ///
    /// Will panic if the ``ChatCompletionRequestAssistantMessageArgs`` cannot be created
    pub fn push_stream(&mut self, text: &str, first: bool) {
        self.current_response += &text;
        if !first {
            self.history.pop();
        }
        self.history.push(ChatCompletionRequestMessage::Assistant(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(&self.current_response)
                .build()
                .unwrap(),
        ));
    }

    pub fn is_empty(&mut self) -> bool {
        self.history.is_empty()
    }

    pub fn clear_message(&mut self) {
        self.current_response = String::new();
    }
}
