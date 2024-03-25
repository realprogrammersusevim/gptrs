use crate::api::{ChatMessage, Role};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};
use textwrap::wrap;

use crate::config::Prompt;

#[derive(Default, Clone, Debug)]
pub struct History {
    pub history: Vec<ChatMessage>,
    pub current_response: String,
    pub text_width: u16,
    pub text_lines: usize,
    pub tokens: usize,
}

impl History {
    /// # Panics
    ///
    /// Will panic if the response somehow contains something other than text
    pub fn render_history(&mut self) -> Vec<Line> {
        let mut message_text = vec![];
        for message in &self.history {
            match &message.role {
                Role::User => {
                    let text = &message.content;
                    let wrapped = wrap(text, self.text_width as usize);
                    for line in wrapped {
                        message_text.push(Line::styled(
                            line.to_string(),
                            Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                Role::Assistant => {
                    let text = &message.content;
                    let wrapped = wrap(text, self.text_width as usize);
                    for line in wrapped {
                        message_text
                            .push(Line::styled(line.to_string(), Style::new().bg(Color::Red)));
                    }
                }
                Role::System => {}
            }
        }

        self.text_lines = message_text.len();

        message_text
    }

    /// # Panics
    ///
    /// Will panic if the ``ChatCompletionRequestMessage`` cannot be created with the ``Prompt``
    /// content
    pub fn push(&mut self, prompt: Prompt) {
        self.history.push(prompt.into());
    }

    pub fn extend(&mut self, prompts: Vec<Prompt>) {
        let messages: Vec<ChatMessage> = prompts.into_iter().map(ChatMessage::from).collect();
        self.history.extend(messages);
    }

    /// # Panics
    ///
    /// Will panic if the ``ChatCompletionRequestAssistantMessageArgs`` cannot be created
    pub fn push_stream(&mut self, text: &str, first: bool) {
        self.current_response += &text;
        if first {
            self.history.push(ChatMessage::new(
                self.current_response.clone(),
                Role::Assistant,
            ));
        } else {
            self.history.last_mut().unwrap().content = self.current_response.clone();
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.history.is_empty()
    }

    pub fn clear_message(&mut self) {
        self.current_response = String::new();
    }

    pub fn num_tokens(&mut self, model: &str) -> usize {
        let mut messages = vec![];
        for message in &self.history {
            messages.push(message.clone().into());
        }
        tiktoken_rs::num_tokens_from_messages(model, &messages).unwrap()
    }

    pub fn last(&mut self) -> Option<&ChatMessage> {
        self.history.last()
    }
}
