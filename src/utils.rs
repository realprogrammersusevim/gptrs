use log::LevelFilter;

pub fn initialize_logger() {
    tui_logger::init_logger(LevelFilter::Trace).unwrap_or_else(|_| {
        panic!("Could not initialize the logger");
    });
    tui_logger::set_default_level(log::LevelFilter::Trace);
}
