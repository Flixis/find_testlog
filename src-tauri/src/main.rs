// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use colored::*;
use std::env;

mod functions;
mod structs;

/*
Written 07/09/2023
Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.
*/

pub mod estructs{
    use serde::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchInfo {
        pub drive_letter: String,
        pub folder_location: String,
        pub pn: String,
        pub test_env: String,
        pub sn: String,
        pub year_week: String,
    }
}


#[tauri::command]
fn rust_parse_search_data(pn: String , sn: String, year_week: String, test_env: String) -> String {
    
    let data = estructs::SearchInfo {
        drive_letter: "".to_string(),
        folder_location: "".to_string(),
        pn : pn,
        test_env: test_env,
        sn: sn,
        year_week: year_week,
    };
    
    format!("data:, {:?} from Rust!", data)
}

fn main() {

    /* This part is just so we can keep support for the old CLI features
    We check if there are no arguments, if there are none we launch GUI. */
    let argumentation: Vec<String> = env::args().collect();

    if argumentation.len() < 2{
        tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rust_parse_search_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }


    let default_app_config = structs::AppConfig::default_values();
    let args = structs::Cli::parse();

    // Returns the config location
    if args.get_config_location {
        let file = confy::get_configuration_file_path("find_testlog", None).unwrap();
        eprintln!("{} {:#?}", "Configuration file is located at:".green().bold(), file);
        return;
    }

    // Extract arguments or use default values
    let drive_letter = args.drive_letter.as_deref().unwrap_or(&default_app_config.drive_letter).to_string();
    let folder_location = args.folder_location.as_deref().unwrap_or(&default_app_config.folder_location).to_string();
    let pn = args.pn.as_deref().unwrap_or(&default_app_config.pn).to_string();
    let test_env = args.test_env.as_deref().unwrap_or(&default_app_config.test_env).to_string();

    // Build the folder path, used for get_most_recent_folder_name
    let mut folder_path = format!("{}\\{}\\{}\\", drive_letter, folder_location, pn);
    let year_week = args.year_week.as_deref().unwrap_or("");

    let sn = args.sn.clone().unwrap_or(default_app_config.sn);
    
    if sn.is_empty() {
        eprintln!("{}", "SN cannot be empty".red().bold());
        return; // Exit the app;
    }

    // Build the folder path, this time with all of its values to parse for finding the log file.
    if args.year_week.is_none() {
        // If no year-week is passed, we just pass the normal folder_path
        println!("{}", "Year-week not specified, searching all folders.".green().bold());
    } else {
        println!("{}{}", "Searching inside: ".green().bold(), year_week);
        folder_path = format!("{}\\{}\\{}\\{}\\{}", drive_letter, folder_location, pn, year_week, test_env);
    }

    let app_config = structs::AppConfig { //save current params to cfg file
        drive_letter,
        folder_location,
        pn,
        test_env,
        sn: sn.clone(),
    };
    
    if let Err(err) = app_config.save() {
        eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
    }

    let get_log_file_path = functions::itter_find_log(folder_path, args.clone());
    match get_log_file_path {
        Ok(paths) => {
            if paths.is_empty() {
                println!("{}","No matches found".red().bold());
            } else {
                println!("{}", "Matched log file paths:".green().bold());
                for path in paths {
                    println!("{}", path);
                }
            }
        }
        Err(err) => eprintln!("{} {}", "Error:".red().bold(), err),
    }
}
