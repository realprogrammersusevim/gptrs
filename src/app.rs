use crate::api::stream_chat_completion;
use crate::api::Role;
use crate::config::Prompt;
use crate::event::Event;
use crate::input::{Mode, StyledTextArea, Transition, Vim};
use crate::widgets::error::PopupMessage;
use crate::widgets::error::Severity;
use crate::{chat::History, config::Final};
use clippers::Clipboard;
use crossterm::event::KeyEvent;
use log::warn;
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use tui_logger::TuiWidgetState;
use tui_textarea::TextArea;

#[allow(clippy::module_name_repetitions)]
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Application.
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// App configuration
    pub config: Final,
    /// input editor
    pub input_editor: TextArea<'a>,
    /// the vim mode handler
    pub vim: Vim,
    /// how much to scroll text
    pub chat_scroll: (u16, u16),
    /// the text of the chat
    pub chat_text: History,
    /// Is GPT currently generating text?
    pub generating: bool,
    /// logger widget state
    pub debug_state: TuiWidgetState,
    // if we have an error message
    pub error: Option<PopupMessage>,
}

impl Default for App<'_> {
    fn default() -> Self {
        let config = Final::default();
        let mut def = Self {
            running: true,
            config: config.clone(),
            input_editor: StyledTextArea::styled_default(),
            vim: Vim::new(Mode::Normal),
            chat_scroll: (0, 0),
            chat_text: History::default(),
            generating: false,
            debug_state: TuiWidgetState::default(),
            error: None,
        };

        def.chat_text.extend(config.prompt);

        if config.vim {
            def.input_editor.set_block(
                StyledTextArea::styled_default()
                    .block()
                    .unwrap()
                    .clone()
                    .title(def.vim.mode.to_string()),
            );
        }

        def
    }
}

impl App<'_> {
    /// Constructs a new instance of [`App`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub const fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn scroll_down(&mut self, by_lines: u16) {
        if self.chat_scroll.0 < u16::try_from(self.chat_text.text_lines).unwrap_or(10) - (by_lines)
        {
            self.chat_scroll.0 += by_lines;
        }
    }

    pub fn scroll_up(&mut self, by_lines: u16) {
        if self.chat_scroll.0 > (by_lines - 1) {
            self.chat_scroll.0 -= by_lines;
        }
    }

    /// # Panics
    ///
    /// Will panic if ``StyledTextArea`` cannot be created
    pub fn edit_input(&mut self, input: KeyEvent) {
        if self.config.vim {
            self.vim = match self
                .vim
                .transition(StyledTextArea::into_input(input), &mut self.input_editor)
            {
                Transition::Mode(mode) if self.vim.mode != mode => {
                    self.input_editor.set_block(
                        StyledTextArea::styled_default()
                            .block()
                            .unwrap()
                            .clone()
                            .title(mode.to_string()),
                    );
                    Vim::new(mode)
                }
                Transition::Nop | Transition::Mode(_) => self.vim.clone(),
                Transition::Pending(input) => self.vim.clone().with_pending(input),
            }
        } else {
            self.input_editor.input(StyledTextArea::into_input(input));
        }
    }

    pub fn append_message(&mut self) {
        if !self.generating {
            self.chat_text.push(Prompt {
                role: Role::User,
                content: StyledTextArea::text(&mut self.input_editor),
            });

            self.input_editor = StyledTextArea::styled_default();

            if self.config.vim {
                self.input_editor.set_block(
                    StyledTextArea::styled_default()
                        .block()
                        .unwrap()
                        .clone()
                        .title(self.vim.mode.to_string()),
                );
            }
        }
    }

    /// # Panics
    ///
    /// Will panic if a request cannot be created
    ///
    /// # Errors
    ///
    /// Returns Ok
    pub fn start_generation(&mut self, sender: mpsc::Sender<Event>) -> Result<(), Box<dyn Error>> {
        let model = self.config.model.clone();
        let messages = self.chat_text.history.clone();
        let key = self.config.api_key.clone();
        let base = self.config.api_base.clone();
        thread::spawn(move || {
            stream_chat_completion(messages, &key, &base, &model, sender.clone());
            sender.send(Event::EndGeneration).unwrap();
        });

        Ok(())
    }

    pub fn reset_history(&mut self) {
        self.chat_text = History::default();
        self.chat_text.extend(self.config.prompt.clone());
        self.chat_scroll = (0, 0);
    }

    pub fn copy_last_message(&mut self, sender: &mpsc::Sender<Event>) {
        let last = self.chat_text.last().unwrap();
        let last_text = &last.clone().to_string();

        let did_write = Clipboard::get().write_text(last_text);

        match did_write {
            Ok(()) => {}
            Err(err) => {
                sender
                    .send(Event::ErrorPopup(
                        Severity::Error,
                        format!("Couldn't copy last message to clipboard: {err:?}"),
                    ))
                    .unwrap();
            }
        }
    }
}
