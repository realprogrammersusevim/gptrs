use gptrs::app::{App, AppResult};
use gptrs::event::{Event, Handler};
use gptrs::handler::{handle_end, handle_key_events, handle_new_message, handle_token};
use gptrs::handler::{handle_mouse_events, handle_start_generation};
use gptrs::tui::Tui;
use log::{debug, LevelFilter};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fs::create_dir;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    if app.config.debug {
        // Set up our logging
        let log_dir = dirs::data_dir().unwrap().join("gptrs");
        if !log_dir.exists() {
            create_dir(log_dir.clone()).unwrap_or_else(|_| {
                panic!(
                    "Could not create the the logging directory {}",
                    log_dir.display()
                )
            });
        }
        simple_logging::log_to_file(log_dir.join("test.log"), LevelFilter::Debug).unwrap();
    }

    debug!("{:?}", app);

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
            event = tui.events.next().await?;
        }

        debug!("The event we're handling is {:?}", event);

        match event {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app, tui.events.sender()).await?;
            }
            Event::Mouse(mouse_event) => {
                // If there are two mouse events we can handle both of them at once.
                let first = mouse_event;
                let second = tui.events.next().await?;
                peeked = true;
                peeked_event = second.clone();
                match second {
                    Event::Mouse(second_mouse) => {
                        handle_mouse_events(first, Some(second_mouse), &mut app)?
                    }
                    _ => handle_mouse_events(first, None, &mut app)?,
                }
            }
            Event::Message => handle_new_message(&mut app, tui.events.sender()).await?,
            Event::StartGeneration => {
                handle_start_generation(&mut app, tui.events.sender()).await?;
            }
            Event::Token(token, first) => handle_token(&mut app, &token, first)?,
            Event::EndGeneration => handle_end(&mut app)?,
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
