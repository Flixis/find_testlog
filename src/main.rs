use clap::Parser;
use colored::*;
use std::process;

mod functions;
mod structs;

/*
Written 15/07/2023
Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.
*/

fn main() {
    let default_app_config = structs::AppConfig::default_values();
    let args = structs::Cli::parse();

    // Returns the config location
    if args.get_config_location {
        let file = confy::get_configuration_file_path("find_testlog", None).unwrap();
        println!("{} {:#?}", "Configuration file is located at:".red().bold(), file);
        return;
    }

    // Extract arguments or use default values
    let drive_letter = args.drive_letter.as_ref().unwrap_or(&default_app_config.drive_letter).to_string();
    let folder_location = args.folder_location.as_ref().unwrap_or(&default_app_config.folder_location).to_string();
    let pn = args.pn.as_ref().unwrap_or(&default_app_config.pn).to_string();
    let test_env = args.test_env.as_ref().unwrap_or(&default_app_config.test_env).to_string();

    // Build the folder path, used for get_most_recent_folder_name
    let folder_path = format!("{}\\{}\\{}\\", drive_letter, folder_location, pn);
    let latest_year_week = functions::get_most_recent_folder_name(&folder_path);
    let year_week = args.year_week.as_ref().unwrap_or(&latest_year_week);

    let sn = args.sn.clone().unwrap_or(default_app_config.sn);
    
    if sn.is_empty() {
        eprintln!("{}", "SN cannot be empty".red().bold());
        process::exit(1);
    }

    // Build the folder path, this time with all of its values to parse for finding the log file.
    let folder_path = format!("{}\\{}\\{}\\{}\\{}", drive_letter, folder_location, pn, year_week, test_env);

    let app_config = structs::AppConfig {
        drive_letter,
        folder_location,
        pn,
        test_env,
        sn: sn.clone(),
    };
    
    if let Err(err) = app_config.save() {
        eprintln!("Failed to save configuration: {}", err);
    }

    functions::itter_find_log(folder_path, args.clone());
}

