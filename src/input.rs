use core::fmt;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    text::Line,
    widgets::{Block, BorderType, Borders},
};
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Operator(char),
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Insert => write!(f, "Insert"),
            Mode::Visual => write!(f, "Visual"),
            Mode::Operator(c) => write!(f, "Operator ({})", c),
        }
    }
}

pub enum Transition {
    Nop,
    Mode(Mode),
    Pending(Input),
}

#[derive(Debug, Clone)]
pub struct Vim {
    pub mode: Mode,
    pub pending: Input,
}

impl Vim {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            pending: Input::default(),
        }
    }

    pub fn with_pending(self, pending: Input) -> Self {
        Self {
            mode: self.mode,
            pending,
        }
    }

    pub fn transition(&self, input: Input, textarea: &mut TextArea<'_>) -> Transition {
        if input.key == Key::Null {
            return Transition::Nop;
        }

        match self.mode {
            Mode::Normal | Mode::Visual | Mode::Operator(_) => {
                match input {
                    Input {
                        key: Key::Char('h'),
                        ..
                    } => textarea.move_cursor(CursorMove::Back),
                    Input {
                        key: Key::Char('j'),
                        ..
                    } => textarea.move_cursor(CursorMove::Down),
                    Input {
                        key: Key::Char('k'),
                        ..
                    } => textarea.move_cursor(CursorMove::Up),
                    Input {
                        key: Key::Char('l'),
                        ..
                    } => textarea.move_cursor(CursorMove::Forward),
                    Input {
                        key: Key::Char('w'),
                        ..
                    } => textarea.move_cursor(CursorMove::WordForward),
                    Input {
                        key: Key::Char('b'),
                        ctrl: false,
                        ..
                    } => textarea.move_cursor(CursorMove::WordBack),
                    Input {
                        key: Key::Char('^'),
                        ..
                    } => textarea.move_cursor(CursorMove::Head),
                    Input {
                        key: Key::Char('$'),
                        ..
                    } => textarea.move_cursor(CursorMove::End),
                    Input {
                        key: Key::Char('D'),
                        ..
                    } => {
                        textarea.delete_line_by_end();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('C'),
                        ..
                    } => {
                        textarea.delete_line_by_end();
                        textarea.cancel_selection();
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('p'),
                        ..
                    } => {
                        textarea.paste();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('u'),
                        ctrl: false,
                        ..
                    } => {
                        textarea.undo();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('r'),
                        ctrl: true,
                        ..
                    } => {
                        textarea.redo();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('x'),
                        ..
                    } => {
                        textarea.delete_next_char();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('i'),
                        ..
                    } => {
                        textarea.cancel_selection();
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('a'),
                        ..
                    } => {
                        textarea.cancel_selection();
                        textarea.move_cursor(CursorMove::Forward);
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('A'),
                        ..
                    } => {
                        textarea.cancel_selection();
                        textarea.move_cursor(CursorMove::End);
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('o'),
                        ..
                    } => {
                        textarea.move_cursor(CursorMove::End);
                        textarea.insert_newline();
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('O'),
                        ..
                    } => {
                        textarea.move_cursor(CursorMove::Head);
                        textarea.insert_newline();
                        textarea.move_cursor(CursorMove::Up);
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('I'),
                        ..
                    } => {
                        textarea.cancel_selection();
                        textarea.move_cursor(CursorMove::Head);
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('e'),
                        ctrl: true,
                        ..
                    } => textarea.scroll((1, 0)),
                    Input {
                        key: Key::Char('y'),
                        ctrl: true,
                        ..
                    } => textarea.scroll((-1, 0)),
                    Input {
                        key: Key::Char('d'),
                        ctrl: true,
                        ..
                    } => textarea.scroll(Scrolling::HalfPageDown),
                    Input {
                        key: Key::Char('u'),
                        ctrl: true,
                        ..
                    } => textarea.scroll(Scrolling::HalfPageUp),
                    Input {
                        key: Key::Char('f'),
                        ctrl: true,
                        ..
                    } => textarea.scroll(Scrolling::PageDown),
                    Input {
                        key: Key::Char('b'),
                        ctrl: true,
                        ..
                    } => textarea.scroll(Scrolling::PageUp),
                    Input {
                        key: Key::Char('v'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Normal => {
                        textarea.start_selection();
                        return Transition::Mode(Mode::Visual);
                    }
                    Input {
                        key: Key::Char('V'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Normal => {
                        textarea.move_cursor(CursorMove::Head);
                        textarea.start_selection();
                        textarea.move_cursor(CursorMove::End);
                        return Transition::Mode(Mode::Visual);
                    }
                    Input { key: Key::Esc, .. }
                    | Input {
                        key: Key::Char('v'),
                        ctrl: false,
                        ..
                    } => {
                        if self.mode == Mode::Visual {
                            textarea.cancel_selection();
                        }
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('g'),
                        ctrl: false,
                        ..
                    } if matches!(
                        self.pending,
                        Input {
                            key: Key::Char('g'),
                            ctrl: false,
                            ..
                        }
                    ) =>
                    {
                        textarea.move_cursor(CursorMove::Top)
                    }
                    Input {
                        key: Key::Char('G'),
                        ctrl: false,
                        ..
                    } => textarea.move_cursor(CursorMove::Bottom),
                    Input {
                        key: Key::Char(c),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Operator(c) => {
                        // Handle yy, dd, cc. (This is not strictly the same behavior as Vim)
                        textarea.move_cursor(CursorMove::Head);
                        textarea.start_selection();
                        let cursor = textarea.cursor();
                        textarea.move_cursor(CursorMove::Down);
                        if cursor == textarea.cursor() {
                            textarea.move_cursor(CursorMove::End); // At the last line, move to end of the line instead
                        }
                    }
                    Input {
                        key: Key::Char(op @ ('y' | 'd' | 'c')),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Normal => {
                        textarea.start_selection();
                        return Transition::Mode(Mode::Operator(op));
                    }
                    Input {
                        key: Key::Char('y'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Visual => {
                        textarea.copy();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('d'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Visual => {
                        textarea.cut();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('c'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Visual => {
                        textarea.cut();
                        return Transition::Mode(Mode::Insert);
                    }
                    input => return Transition::Pending(input),
                }

                // Handle the pending operator
                match self.mode {
                    Mode::Operator('y') => {
                        textarea.copy();
                        Transition::Mode(Mode::Normal)
                    }
                    Mode::Operator('d') => {
                        textarea.cut();
                        Transition::Mode(Mode::Normal)
                    }
                    Mode::Operator('c') => {
                        textarea.cut();
                        Transition::Mode(Mode::Insert)
                    }
                    _ => Transition::Nop,
                }
            }
            Mode::Insert => match input {
                Input { key: Key::Esc, .. } => Transition::Mode(Mode::Normal),
                input => {
                    textarea.input_without_shortcuts(input); // Don't use the default mappings
                    Transition::Mode(Mode::Insert)
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct StyledTextArea<'a>(TextArea<'a>);

impl<'a> StyledTextArea<'_> {
    pub fn default() -> TextArea<'a> {
        let mut editor = TextArea::default();
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        editor.set_block(block);
        editor
    }

    pub fn into_input(event: KeyEvent) -> Input {
        match event.code {
            KeyCode::Backspace => Input {
                key: Key::Backspace,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Enter => Input {
                key: Key::Enter,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Left => Input {
                key: Key::Left,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Up => Input {
                key: Key::Up,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Down => Input {
                key: Key::Down,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Home => Input {
                key: Key::Home,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::End => Input {
                key: Key::End,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::PageUp => Input {
                key: Key::PageUp,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::PageDown => Input {
                key: Key::PageDown,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Tab => Input {
                key: Key::Tab,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::BackTab => Input {
                key: Key::Null,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Delete => Input {
                key: Key::Delete,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Insert => Input {
                key: Key::Null,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            KeyCode::Esc => Input {
                key: Key::Esc,
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: false,
            },
            KeyCode::Char(char) => Input {
                key: Key::Char(char),
                alt: event.modifiers.contains(KeyModifiers::ALT),
                ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
            },
            _ => Input {
                key: Key::Null,
                ctrl: false,
                shift: event.modifiers.contains(KeyModifiers::SHIFT),
                alt: false,
            },
        }
    }

    pub fn tui_lines(textarea: &mut TextArea) -> Vec<Line<'a>> {
        let lines = textarea.lines();
        let mut tui_lines = vec![];
        for line in lines {
            tui_lines.push(Line::from(line.to_string()))
        }

        tui_lines
    }

    pub fn text(textarea: &mut TextArea) -> String {
        textarea.lines().join("\n")
    }
}
