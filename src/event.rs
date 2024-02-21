use crate::app::AppResult;
use crate::widgets::error::Severity;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use log::error;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task;

/// Terminal events.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Append new user message
    Message,
    /// Begin generating Assistant response
    StartGeneration,
    /// New token
    Token(String, bool),
    /// End the assistant generation
    EndGeneration,
    /// Terminal resize.
    Resize(u16, u16),
    /// Show an error popup.
    ErrorPopup(Severity, String),
    /// Clear the error popup.
    ClearErrorPopup,
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Handler {
    /// Event sender channel.
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    handler: task::JoinHandle<()>,
}

impl Handler {
    #[must_use]
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel(256);
        let handler = {
            let sender = sender.clone();
            tokio::spawn(async move {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .await
                        .expect("failed to send terminal event");
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if matches!(sender.send(Event::Tick).await, Ok(())) {
                            last_tick = Instant::now();
                        } else {
                            error!("Couldn't send the tick event. Assuming the app shut down.");
                            return;
                        }
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> AppResult<Event> {
        Ok(self.receiver.recv().await.unwrap())
    }

    #[must_use]
    pub fn sender(&self) -> mpsc::Sender<Event> {
        self.sender.clone()
    }

    pub async fn send(&self, event: Event) -> Result<(), mpsc::error::SendError<Event>> {
        self.sender.send(event).await
    }
}
