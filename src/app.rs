use crate::chat::ChatHistory;
use crate::config::Config;
use crate::event::Event;
use crate::input::StyledTextArea;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::Client;
use crossterm::event::KeyEvent;
use futures::StreamExt;
use std::error::Error;
use tokio::sync::mpsc;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// App configuration
    pub config: Config,
    /// input editor
    pub input_editor: StyledTextArea<'a>,
    /// how much to scroll text
    pub chat_scroll: (u16, u16),
    /// the text of the chat
    pub chat_text: ChatHistory,
    /// Is GPT currently generating text?
    pub generating: bool,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            running: true,
            config: Config::default(),
            input_editor: StyledTextArea::default(),
            chat_scroll: (0, 0),
            chat_text: ChatHistory::default(),
            generating: false,
        }
    }
}

impl App<'_> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn scroll_down(&mut self) {
        if self.chat_scroll.0 < self.chat_text.len() as u16 - 1 {
            self.chat_scroll.0 += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.chat_scroll.0 > 0 {
            self.chat_scroll.0 -= 1;
        }
    }

    pub fn edit_input(&mut self, input: KeyEvent) {
        self.input_editor.input_event(input);
    }

    pub fn append_message(&mut self) {
        if !self.generating {
            self.chat_text.push(self.input_editor.text());
            self.input_editor = StyledTextArea::default();
        }
    }

    pub async fn start_generation(
        &mut self,
        sender: mpsc::Sender<Event>,
    ) -> Result<(), Box<dyn Error>> {
        let model = self.config.model.clone().unwrap();
        let messages = self.chat_text.history.clone();
        tokio::spawn(async move {
            let client = Client::new();

            let request = CreateChatCompletionRequestArgs::default()
                .model(model)
                .max_tokens(512u16)
                .messages(messages)
                .build()
                .unwrap();
            let mut stream = client.chat().create_stream(request).await.unwrap();
            let mut first = true;
            while let Some(result) = stream.next().await {
                match result {
                    Ok(response) => {
                        for chat_choice in response.choices.iter() {
                            if let Some(ref content) = chat_choice.delta.content {
                                sender
                                    .send(Event::Token(content.to_string(), first))
                                    .await
                                    .unwrap();
                                first = false;
                            }
                        }
                    }
                    Err(res) => panic!("Error: returned response {:?}", res),
                }
            }

            sender.send(Event::EndGeneration).await.unwrap();
        });

        Ok(())
    }
}
