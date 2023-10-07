// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use colored::*;
use log::{debug, error};
use serde_json::{json, Value};
use std::process::exit;
use std::{thread, time::Duration};

mod functions;
mod structs;

/*
(C) Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.
*/

/*

Yes, the main call of the app is not clean.
But hey... it works...

I may fix this later. But for now it serves its purpose.

*/
fn main() {
    // Builds the Tauri connection
    tauri::Builder::default()
        .setup(|app| {
            //Load current config, if nothing is availible just load defaults.
            let mut search_info = structs::AppConfig::default_values();
            let mut cli_enabled = false;
            search_info.open_log = false; //by default make sure the log is not opened.
                                          // Default to GUI if the app was opened with no CLI args.
            if std::env::args_os().count() <= 1 && !cli_enabled {
                cli_gui(app.handle())?;
            }
            // Else, we start in CLI mode and parse the given parameters
            let matches = match app.get_cli_matches() {
                Ok(matches) => matches,
                Err(err) => {
                    error!("{}", err);
                    app.handle().exit(1);
                    return Ok(());
                }
            };
            // Iterate over each key and execute functions based on them
            for (key, data) in matches.args {
                if data.occurrences > 0 || key.as_str() == "help" {
                    cli_enabled = true;
                    match key.as_str() {
                        "pn" => {
                            // Create a new SearchInfo struct with only the pn field set
                            let saved_to_struct =
                                functions::strip_string_of_leading_and_trailing_slashes(data);
                            search_info.productnumber = saved_to_struct;
                        }
                        "sn" => {
                            // Create a new SearchInfo struct with only the sn field set
                            let saved_to_struct =
                                functions::strip_string_of_leading_and_trailing_slashes(data);
                            search_info.serialnumber = saved_to_struct;
                        }
                        "year_week" => {
                            // Create a new SearchInfo struct with only the year_week field set
                            let saved_to_struct =
                                functions::strip_string_of_leading_and_trailing_slashes(data);
                            search_info.dateyyyyww = saved_to_struct;
                        }
                        "test_env" => {
                            // Create a new SearchInfo struct with only the test_env field set
                            let saved_to_struct =
                                functions::strip_string_of_leading_and_trailing_slashes(data);
                            search_info.test_env = saved_to_struct;
                        }
                        "open_log" => {
                            search_info.open_log = data.value.is_boolean();
                        }
                        "drive_letter" => {
                            // Set the drive_letter field
                            let saved_to_struct =
                                functions::strip_string_of_leading_and_trailing_slashes(data);
                            search_info.drive_letter = saved_to_struct;
                        }
                        "folder_location" => {
                            // Set the folder_location field
                            let saved_to_struct =
                                functions::strip_string_of_leading_and_trailing_slashes(data);
                            search_info.folder_location = saved_to_struct;
                        }
                        "get_config_file" => {
                            // Set the get_config_location flag to true
                            get_configuration_file_path("find_testlog");
                            exit(2);
                        }
                        _ => functions::not_implemented(app.handle()),
                    }
                }
            }

            //Make sure to save after we've written new data
            if let Err(err) = search_info.save() {
                eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
            }

            //this if-statment is important, if you remove it will always start a search, thus delaying GUI launch.
            if cli_enabled {
                if search_info.serialnumber.is_empty() && cli_enabled {
                    eprintln!("{}", "SN cannot be empty".red().bold());
                    exit(2);
                }

                let get_log_file_path = functions::search_for_log(&search_info);
                match get_log_file_path {
                    Ok(paths) => {
                        if paths.is_empty() {
                            println!("{} {:?}", "No matches found: ".red().bold(), search_info);
                        } else {
                            println!("{}", "Matched log file paths:".green().bold());
                            for path in paths {
                                println!("{}", path);
                            }
                        }
                    }
                    Err(err) => eprintln!("{} {} \n {:?}", "Error:".red().bold(), err, search_info),
                }

                exit(0);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            parse_frontend_search_data,
            get_configuration_file_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

/*

Parsing data from frontend.
The search logic for the GUI part of the app is done here.


*/

#[tauri::command]
fn parse_frontend_search_data(
    productnumber: Option<String>,
    serialnumber: Option<String>,
    dateyyyyww: Option<String>,
    testenv: Option<String>,
) -> Value {
    let mut search_info = structs::AppConfig::default_values();

    search_info.serialnumber = serialnumber.unwrap();
    search_info.productnumber = productnumber.unwrap();
    search_info.dateyyyyww = dateyyyyww.unwrap();
    search_info.test_env = testenv.unwrap();

    let mut results_from_search_json: Value = json!({
        "datetime": [],
        "testenv": [],
        "location": [],
        "serialnumber": [],
    });

    //Make sure to save after we've written new data
    if let Err(err) = search_info.save() {
        eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
    }

    let log_file_path = functions::search_for_log(&search_info);
    match log_file_path {
        Ok(paths) => {
            if paths.is_empty() {
                eprintln!(
                    "{} {:?}",
                    "Path could not be matched".red().bold(),
                    search_info
                );
            } else {
                for path in paths {
                    dbg!(&path);
                    let extracted_datetime = functions::extract_datetime(&path);
                    let extracted_test_env = functions::get_test_env_string(&path);
                    let mut _json_data: Value = json!({
                        "datetime": extracted_datetime,
                        "testenv": extracted_test_env,
                        "location": path.to_string(),
                        "serialnumber": search_info.serialnumber,
                    });
                    dbg!(&_json_data);
                    // Push values to arrays in the JSON object
                    results_from_search_json["datetime"]
                        .as_array_mut()
                        .unwrap()
                        .push(_json_data["datetime"].take());
                    results_from_search_json["testenv"]
                        .as_array_mut()
                        .unwrap()
                        .push(_json_data["testenv"].take());
                    results_from_search_json["location"]
                        .as_array_mut()
                        .unwrap()
                        .push(_json_data["location"].take());
                    results_from_search_json["serialnumber"]
                        .as_array_mut()
                        .unwrap()
                        .push(_json_data["serialnumber"].take());
                }
            }
        }
        _ => eprintln!("{} {:?}", "No matches found: ".red().bold(), search_info),
    }

    results_from_search_json
}

/*

Gets the path to the configuration file.
Why is this here and not in functions.rs?
Because the #[tauri::command] complains, and I cba to figure it out now.
TODO: Fix this.

*/
#[tauri::command]
fn get_configuration_file_path(confy_config_name: &str) -> std::path::PathBuf {
    match confy::get_configuration_file_path(&confy_config_name, None) {
        Ok(file) => {
            println!(
                "{} {:#?}",
                "Configuration file is located at:".green().bold(),
                file
            );
            return file;
        }
        Err(err) => {
            eprintln!("Failed to get configuration file path: {}", err);
            exit(1)
        }
    };
}

/*

Create a GUI with following options.

*/
fn cli_gui(app: tauri::AppHandle) -> Result<(), tauri::Error> {
    debug!("showing gui");
    println!(
        "{}",
        "Starting Test Log Finder! Tariq Dinmohamed (C)"
            .green()
            .bold()
    );
    functions::hide_windows_console(false); //<--- this function should be take a bool, I want the user to be able to see the CLI if they desire.
    thread::sleep(Duration::from_millis(700)); //Here because sometimes the console window is removed before the GUI renders, killing the app.
    tauri::WindowBuilder::new(
        &app,
        "FindTestlog",
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("Find Testlog")
    .inner_size(800., 480.)
    .resizable(true)
    .build()?;
    Ok(())
}
