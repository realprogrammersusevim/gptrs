use ratatui::{
    layout::{Alignment, Layout},
    prelude::{Constraint, Direction},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
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

    let mut chat_area = main_layout[1];
    let mut debug_area = main_layout[1];

    if app.config.debug {
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);
        chat_area = inner_layout[0];
        debug_area = inner_layout[1];
    }

    // Title widget
    let title = Paragraph::new(format!(
        "Model: {}\n\
        API key: {}\n\
        Tokens: {}",
        app.config.model,
        mask_api_key(&app.config.api_key, 5),
        app.chat_text.tokens
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
    app.chat_text.text_width = chat_area.width - 2;
    let chat_list = Paragraph::new(app.chat_text.render_history())
        .scroll(app.chat_scroll)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

    frame.render_widget(chat_list, chat_area);

    if app.config.debug {
        let tui_sm = TuiLoggerWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .state(&app.debug_state)
            .block(Block::default().borders(Borders::RIGHT));
        frame.render_widget(tui_sm, debug_area);
    }

    // Input widget
    let chat_input = app.input_editor.widget();

    frame.render_widget(chat_input, main_layout[2]);

    if app.error.is_some() {
        frame.render_widget(app.error.clone().unwrap(), frame.size());
    }
}

/// Mask a displayed API key from shoulder snoopers
///
/// * `api_key` - The API key to mask
/// * `visible` - The number of characters to leave visible at the beginning
fn mask_api_key(api_key: &str, visible: usize) -> String {
    api_key[0..visible].to_owned() + &"*".repeat(api_key.len() - visible)
}
