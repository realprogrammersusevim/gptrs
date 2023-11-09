use crossterm::event::KeyEvent;
use ratatui::text::Line;
use std::error;

use crate::input::StyledTextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// input editor
    pub input_editor: StyledTextArea<'a>,
    /// how much to scroll text
    pub chat_scroll: (u16, u16),
    /// the text of the chat
    pub chat_text: Vec<Line<'a>>,
    /// Is GPT currently generating text?
    pub generating: bool,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            running: true,
            input_editor: StyledTextArea::default(),
            chat_scroll: (0, 0),
            chat_text: vec![],
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
            self.chat_text.append(&mut self.input_editor.tui_lines());
            self.input_editor = StyledTextArea::default();
            self.generating = true;
        }
    }
}
