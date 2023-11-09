use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    text::Line,
    widgets::{Block, BorderType, Borders, Widget},
};
use tui_textarea::{Input, Key, TextArea};

#[derive(Debug)]
pub struct StyledTextArea<'a>(TextArea<'a>);

impl<'a> StyledTextArea<'_> {
    pub fn input_event(&mut self, event: KeyEvent) {
        let input = match event.code {
            KeyCode::Backspace => Input {
                key: Key::Backspace,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Enter => Input {
                key: Key::Enter,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Left => Input {
                key: Key::Left,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Up => Input {
                key: Key::Up,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Down => Input {
                key: Key::Down,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Home => Input {
                key: Key::Home,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::End => Input {
                key: Key::End,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::PageUp => Input {
                key: Key::PageUp,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::PageDown => Input {
                key: Key::PageDown,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Tab => Input {
                key: Key::Tab,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::BackTab => Input {
                key: Key::Null,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Delete => Input {
                key: Key::Delete,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Insert => Input {
                key: Key::Null,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            KeyCode::Char(char) => Input {
                key: Key::Char(char),
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            },
            _ => Input {
                key: Key::Null,
                ctrl: false,
                alt: false,
            },
        };

        self.0.input(input);
    }

    pub fn tui_lines(&mut self) -> Vec<Line<'a>> {
        let lines = self.0.lines();
        let mut tui_lines = vec![];
        for line in lines {
            tui_lines.push(Line::from(line.to_string()))
        }

        tui_lines
    }

    pub fn widget(&mut self) -> impl Widget + '_ {
        self.0.widget()
    }
}

impl Default for StyledTextArea<'_> {
    fn default() -> Self {
        let mut editor = TextArea::default();
        editor.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        Self(editor)
    }
}
