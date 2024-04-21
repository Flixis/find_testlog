use chrono::prelude::*;
use simplelog::*;
use whoami::fallible;
use std::fs::{self, OpenOptions};
use std::path::Path;

/// App logging is setup with the following configuration:
///
/// Terminal logger -> Filter:Warn, Config:Default, TerminalMode: Mixed, ColorChoice: Auto
///
/// Write Logger -> Filter:Info, Config:Default, File: Create(filename)
///
/// filename -> find_testlog_logs/{day-month-year_hour_minute}_{username}_{hostname}_{find_testlog}.log
pub fn setup_loggers() -> Result<(), String> {
    fs::create_dir_all("find_testlog_logs")
        .map_err(|e| format!("unable to create logging directory: {}", e))?;

    let utc = Utc::now().format("%d-%m-%Y_%H_%M").to_string();
    let hostname = match fallible::hostname() {
        Ok(name) => name,
        Err(e) => {
            eprintln!("Failed to get hostname: {}", e);
            "Failed to get hostname".to_string()
        }
    };

    let username = match fallible::username() {
        Ok(name) => name,
        Err(e) => {
            eprintln!("Failed to get username: {}", e);
            "Failed to get username".to_string()
        }
    };

    let filename_string_creation = format!(
        "find_testlog_logs/{}_{}_{}_{}",
        utc,
        username.to_lowercase(),
        hostname.to_lowercase(),
        "find_testlog.log"
    );
    let filename = Path::new(&filename_string_creation);

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)
        .map_err(|e| format!("failed to open or create log file: {}", e))?;

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), file),
    ])
    .map_err(|e| format!("Couldn't initialize loggers: {}", e))?;

    Ok(())
}
