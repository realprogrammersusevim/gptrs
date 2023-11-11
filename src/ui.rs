use ratatui::{
    layout::{Alignment, Layout},
    prelude::{Constraint, Direction},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let area = frame.size();

    // The layout in that main view
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Max(5),
            Constraint::Percentage(70),
            Constraint::Max(6),
        ])
        .split(area);

    // Title widget

    let title = Paragraph::new(format!(
        "Model: {}\n\
        API key: {}\n\
        API base: {}",
        app.config.model.as_ref().unwrap(),
        app.config.api_key.as_ref().unwrap(),
        app.config.api_base.as_ref().unwrap()
    ))
    .block(
        Block::default()
            .title("Information")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(title, main_layout[0]);

    // Chat list widget
    let chat_list = Paragraph::new(app.chat_text.render_history())
        .scroll(app.chat_scroll)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

    frame.render_widget(chat_list, main_layout[1]);

    // Input widget

    let chat_input = app.input_editor.widget();

    frame.render_widget(chat_input, main_layout[2])
}
