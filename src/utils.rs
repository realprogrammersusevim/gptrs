use log::LevelFilter;
use std::fs::create_dir;

pub fn initialize_file_logger() {
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
    simple_logging::log_to_file(log_dir.join("gptrs.log"), LevelFilter::Debug).unwrap();
}

pub fn initialize_logger() {
    tui_logger::init_logger(LevelFilter::Trace).unwrap_or_else(|_| {
        panic!("Could not initialize the logger");
    });
    tui_logger::set_default_level(log::LevelFilter::Trace);
}
