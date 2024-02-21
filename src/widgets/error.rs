use ratatui::prelude::*;
use ratatui::widgets::BorderType;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap};
use std::cmp::min;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Error => write!(f, "Error"),
            Self::Warning => write!(f, "Warning"),
            Self::Info => write!(f, "Info"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PopupMessage {
    pub message: String,
    pub severity: Severity,
    area: Option<Rect>,
}

impl PopupMessage {
    #[must_use]
    pub const fn new(message: String, severity: Severity) -> Self {
        Self {
            message,
            severity,
            area: None,
        }
    }
}

impl Widget for PopupMessage {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let area = if let Some(next) = self.area.take() {
            let width = min(next.width, area.width);
            let height = min(next.height, area.height);
            let x = next.x.clamp(buf.area.x, area.right() - width);
            let y = next.y.clamp(buf.area.y, area.bottom() - height);

            Rect::new(x, y, width, height)
        } else {
            let width = min(area.width, 80);
            let height = min(area.height, 10);
            let x = (area.width - width) / 2;
            let y = (area.height - height) / 2;

            Rect::new(x, y, width, height)
        };

        Clear.render(area, buf);

        let color = match self.severity {
            Severity::Error => Color::Red,
            Severity::Warning => Color::Yellow,
            Severity::Info => Color::Blue,
        };
        let title = match self.severity {
            Severity::Error => "Error",
            Severity::Warning => "Warning",
            Severity::Info => "Info",
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color));
        Paragraph::new(self.message.clone())
            .style(Style::default().fg(color))
            .block(block)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}
