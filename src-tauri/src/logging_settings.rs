
use simplelog::*;
use std::fs::File;
use std::fs;
use std::path::Path;
use whoami;
use chrono::prelude::*;


pub fn setup_loggers(){
    fs::create_dir_all("find_testlog_logs").expect("unable to create logging directory");
    let utc = Utc::now().format("%d-%m-%Y_%H:%M");
    let filename_string_creation = format!("find_testlog_logs/{}_{}_{}_{}", utc ,whoami::username(), whoami::hostname(), "find_testlog.log");
    let filename = Path::new(&filename_string_creation);

    //initialize loggers with settings
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(filename).expect("failed to create log file")),
        ]
    ).expect("Couldn't initialize loggers");    
}