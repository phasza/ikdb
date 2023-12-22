use chrono::Local;
use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::fs;

pub fn configure_logging() -> Result<(), SetLoggerError> {
    // Create a console appender
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build();

    // Get the user's home directory
    let home_dir = dirs::home_dir().expect("Failed to retrieve home directory path");

    // Create a log directory path
    let log_dir_path = home_dir.join("ikdb");

    // Create the log directory if it doesn't exist
    if !log_dir_path.exists() {
        fs::create_dir_all(&log_dir_path).expect("Failed to create log directory");
    }

    // Create a log file path with a timestamp
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let log_file_path = log_dir_path.join(format!("log_{}.log", timestamp));

    // Create a file appender
    let file_appender = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(log_file_path)
        .unwrap(); // Adjust the file name as needed

    // Create a root logger that logs to both console and file
    let root_logger = Root::builder()
        .appender("console")
        .appender("file")
        .build(LevelFilter::Info);

    // Create a log4rs config
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(root_logger)
        .unwrap();

    // Initialize the logger with the config
    log4rs::init_config(config)?;

    Ok(())
}
