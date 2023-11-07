use ratatui::{
    layout::{Alignment, Layout},
    prelude::{Constraint, Direction},
    style::{Color, Style},
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

    // Main view
    let main_block = Block::default()
        .title("GPTrs")
        .borders(Borders::NONE)
        .border_type(BorderType::Rounded);

    let area = frame.size();

    frame.render_widget(main_block, area);

    // The layout in that main view
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Max(6),
            Constraint::Percentage(70),
            Constraint::Max(6),
        ])
        .split(area);

    // Title widget

    let title = Paragraph::new(format!(
        "ChatGPT Information\n\
    Model: none\n\
    API key: none\n\
    API base: none"
    ))
    .block(
        Block::default()
            .title("Info")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(title, main_layout[0]);

    // Chat list widget

    let chat_list = Paragraph::new(app.chat_text.clone())
        .scroll(app.chat_scroll)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

    frame.render_widget(chat_list, main_layout[1]);

    // Input widget

    // let chat_input_text = Paragraph::new(app.key_input.clone()).block(key_block);
    let chat_input_block = Block::default()
        .title("Input")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let chat_input_text = Paragraph::new("Your input").block(chat_input_block);

    frame.render_widget(chat_input_text, main_layout[2])
}
