use crate::widgets::error::{PopupMessage, Severity};
use crate::{
    app::{App, AppResult},
    event::Event,
    input::StyledTextArea,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use std::sync::mpsc;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App<'_>,
    sender: mpsc::Sender<Event>,
) -> AppResult<()> {
    // Clear the error popup if it's visible
    if app.error.is_some() {
        sender.send(Event::ClearErrorPopup).unwrap();
        return Ok(());
    }
    match key_event.code {
        // Exit application on `Ctrl-c`
        KeyCode::Char('c') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            } else {
                app.edit_input(key_event);
            }
        }
        KeyCode::Char('d') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                sender.send(Event::Message).unwrap();
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
        KeyCode::Char('t') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                app.generating = true;
                sender.send(Event::StartGeneration).unwrap();
            } else {
                app.edit_input(key_event);
            }
        }
        KeyCode::Char('x') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.copy_last_message(&sender);
            } else {
                app.edit_input(key_event);
            }
        }
        _ => app.edit_input(key_event),
    }
    Ok(())
}

pub fn handle_new_message(app: &mut App<'_>, sender: &mpsc::Sender<Event>) -> AppResult<()> {
    if app.input_editor.is_empty() {
        return Ok(());
    }
    let text = StyledTextArea::text(&mut app.input_editor);
    app.append_message();
    app.generating = true;
    sender.send(Event::StartGeneration).unwrap();
    let enc = tiktoken_rs::cl100k_base().unwrap();
    let tokens = enc.encode_with_special_tokens(&text);
    app.chat_text.tokens += tokens.len();
    Ok(())
}

pub fn handle_start_generation(app: &mut App<'_>, sender: mpsc::Sender<Event>) -> AppResult<()> {
    if app.config.offline {
        sender
            .send(Event::Token(
                "Running in **offline** mode.".to_string(),
                true,
            ))
            .unwrap();
        sender.send(Event::EndGeneration).unwrap();
    } else {
        app.start_generation(sender)?;
    }

    Ok(())
}

pub fn handle_token(app: &mut App<'_>, token: &str, first: bool) -> AppResult<()> {
    app.chat_text.push_stream(token, first);
    app.chat_text.tokens += 1;

    Ok(())
}

pub fn handle_end(app: &mut App<'_>) -> AppResult<()> {
    app.chat_text.clear_message();
    app.generating = false;

    Ok(())
}

pub fn handle_mouse_events(
    mouse_event: MouseEvent,
    second_event: Option<MouseEvent>,
    app: &mut App<'_>,
) -> AppResult<()> {
    // Don't handle mouse events if the error popup is visible
    if app.error.is_some() {
        return Ok(());
    }

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

pub fn handle_error_popup(app: &mut App<'_>, severity: Severity, message: String) -> AppResult<()> {
    app.error = Some(PopupMessage::new(message, severity));

    Ok(())
}
