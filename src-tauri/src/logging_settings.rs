use chrono::prelude::*;
use simplelog::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use whoami;

/// App logging is setup with the following configuration:
/// 
/// Terminal logger -> Filter:Warn, Config:Default, TerminalMode: Mixed, ColorChoice: Auto
/// 
/// Write Logger -> Filter:Info, Config:Default, File: Create(filename)
/// 
/// filename -> find_testlog_logs/{day-month-year_hour_minute}_{username}_{hostname}_{find_testlog}.log
pub fn setup_loggers() {
    fs::create_dir_all("find_testlog_logs").expect("unable to create logging directory");
    let utc = Utc::now().format("%d-%m-%Y_%H_%M");
    let filename_string_creation = format!(
        "find_testlog_logs/{}_{}_{}_{}",
        utc,
        whoami::username(),
        whoami::hostname(),
        "find_testlog.log"
    );
    let filename = Path::new(&filename_string_creation);

    //initialize loggers with settings
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(filename).expect("failed to create log file"),
        ),
    ])
    .expect("Couldn't initialize loggers");
}
