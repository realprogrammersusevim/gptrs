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
        KeyCode::Char('c') | KeyCode::Char('C') => {
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
        _ => app.edit_input(key_event),
    }
    Ok(())
}

pub async fn handle_new_message(app: &mut App<'_>, sender: mpsc::Sender<Event>) -> AppResult<()> {
    app.append_message();
    app.generating = true;
    sender.send(Event::StartGeneration).await?;

    Ok(())
}

pub async fn handle_start_generation(
    app: &mut App<'_>,
    sender: mpsc::Sender<Event>,
) -> AppResult<()> {
    app.start_generation(sender).await?;

    Ok(())
}

pub fn handle_token(app: &mut App<'_>, token: String, first: bool) -> AppResult<()> {
    app.chat_text.push_stream(token, first);

    Ok(())
}

pub fn handle_end(app: &mut App<'_>) -> AppResult<()> {
    app.chat_text.clear_message();
    app.generating = false;

    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App<'_>) -> AppResult<()> {
    match mouse_event.kind {
        MouseEventKind::ScrollUp => app.scroll_up(),
        MouseEventKind::ScrollDown => app.scroll_down(),
        _ => {}
    }

    Ok(())
}
