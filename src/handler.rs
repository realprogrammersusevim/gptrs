use crate::input::StyledTextArea;
use crate::{
    app::{App, AppResult},
    event::Event,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use tokio::sync::mpsc;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App<'_>,
    sender: mpsc::Sender<Event>,
) -> AppResult<()> {
    match key_event.code {
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            } else {
                app.edit_input(key_event);
            }
        }
        KeyCode::Char('d') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                sender.send(Event::Message).await?;
            } else {
                app.edit_input(key_event);
            }
        }
        KeyCode::Char('r') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                app.reset_history();
            } else {
                app.edit_input(key_event);
            }
        }
        KeyCode::Char('R') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                app.generating = true;
                sender.send(Event::StartGeneration).await?;
            } else {
                app.edit_input(key_event);
            }
        }
        _ => app.edit_input(key_event),
    }
    Ok(())
}

pub async fn handle_new_message(app: &mut App<'_>, sender: mpsc::Sender<Event>) -> AppResult<()> {
    if app.input_editor.is_empty() {
        return Ok(());
    }
    app.append_message();
    app.generating = true;
    sender.send(Event::StartGeneration).await?;
    app.chat_text.tokens = app.chat_text.num_tokens(&app.config.model);

    Ok(())
}

pub async fn handle_start_generation(
    app: &mut App<'_>,
    sender: mpsc::Sender<Event>,
) -> AppResult<()> {
    if app.config.offline {
        sender
            .send(Event::Token(
                "Running in **offline** mode.".to_string(),
                true,
            ))
            .await?;
        sender.send(Event::EndGeneration).await?;
    } else {
        app.start_generation(sender)?;
    }

    Ok(())
}

pub fn handle_token(app: &mut App<'_>, token: &str, first: bool) -> AppResult<()> {
    app.chat_text.push_stream(token, first);

    Ok(())
}

pub fn handle_end(app: &mut App<'_>) -> AppResult<()> {
    app.chat_text.clear_message();
    app.generating = false;
    app.chat_text.tokens = app.chat_text.num_tokens(&app.config.model);

    Ok(())
}

pub fn handle_mouse_events(
    mouse_event: MouseEvent,
    second_event: Option<MouseEvent>,
    app: &mut App<'_>,
) -> AppResult<()> {
    match second_event {
        Some(second) => {
            if mouse_event.kind == MouseEventKind::ScrollUp
                && second.kind == MouseEventKind::ScrollDown
            {
                return Ok(()); // If you scroll up once and down once that cancels out and we don't
                               // move
            } else if mouse_event.kind == MouseEventKind::ScrollDown
                && second.kind == MouseEventKind::ScrollDown
            {
                app.scroll_down(2);
            } else if mouse_event.kind == MouseEventKind::ScrollUp
                && second.kind == MouseEventKind::ScrollUp
            {
                app.scroll_up(2);
            }
        }
        None => match mouse_event.kind {
            MouseEventKind::ScrollUp => app.scroll_up(1),
            MouseEventKind::ScrollDown => app.scroll_down(1),
            _ => {}
        },
    }

    Ok(())
}
