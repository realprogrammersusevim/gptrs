#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Application.
pub mod app;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Terminal events handler.
pub mod event;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Widget renderer.
pub mod ui;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Terminal user interface.
pub mod tui;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Event handler.
pub mod handler;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Configuration
pub mod config;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Text input
pub mod input;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Chat completion
pub mod chat;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Random utility functions
pub mod utils;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// Error widget
pub mod widgets;

#[warn(
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::suspicious,
    clippy::style,
    clippy::correctness,
    clippy::nursery
)]
#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
/// OpenAI API implementation
pub mod api;
