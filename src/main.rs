use gptrs::app::{App, AppResult};
use gptrs::event::{Event, Handler};
use gptrs::handler::{
    handle_end, handle_error_popup, handle_key_events, handle_mouse_events, handle_new_message,
    handle_start_generation, handle_token,
};
use gptrs::tui::Tui;
use gptrs::utils::initialize_logger;
use log::debug;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    if app.config.debug {
        initialize_logger();
    }

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend).unwrap();
    let events = Handler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init().unwrap();

    debug!("Finished with the initialization. Time to start the event loop.");

    let mut peeked = false; // A hack to get around having to actually implement peeking
    let mut peeked_event = Event::Tick;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;

        let mut event = Event::Tick;

        // Handle events.
        if peeked {
            peeked = false;
            event = peeked_event.clone();
        } else {
            event = tui.events.next().unwrap();
        }

        if event != Event::Tick {
            debug!("Handling event {:?}", event);
        }

        match event {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app, tui.events.sender()).unwrap();
            }
            Event::Mouse(first) => {
                // If there are two mouse events we can handle both of them at once.
                let second = tui.events.next().unwrap();
                peeked = true;
                peeked_event = second.clone();
                match second {
                    Event::Mouse(second_mouse) => {
                        debug!(
                            "Handling two mouse events at once, {:?} and {:?}",
                            first, second_mouse
                        );
                        handle_mouse_events(first, Some(second_mouse), &mut app)?
                    }
                    _ => handle_mouse_events(first, None, &mut app)?,
                }
            }
            Event::Message => handle_new_message(&mut app, &tui.events.sender()).unwrap(),
            Event::StartGeneration => {
                handle_start_generation(&mut app, tui.events.sender()).unwrap();
            }
            Event::Token(token, first) => handle_token(&mut app, &token, first)?,
            Event::EndGeneration => handle_end(&mut app)?,
            Event::Resize(_, _) => {}
            Event::ErrorPopup(severity, message) => {
                handle_error_popup(&mut app, severity, message)?
            }
            Event::ClearErrorPopup => {
                app.error = None;
            }
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
