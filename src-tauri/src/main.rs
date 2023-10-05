// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use colored::*;
use log::{debug, error};
use serde_json::{json, Value};
use std::process::exit;
use tauri::{api::shell::open, Manager};

mod functions;
mod structs;

/*
Written 07/09/2023
Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.
*/

/*


IN from JS:
PN SN Year_week test_env

OUT to JS:

Date
Time
Location
SN

How:
Lets search using query string with get_log_file_path() function.
Then we strip the date and time from the path.
Then we encode data as JSON string.

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

    let folder_path: String;
    let mut results_from_search_json: Value = json!({
        "datetime": [],
        "testenv": [],
        "location": [],
        "serialnumber": [],
    });

    dbg!(&search_info);

    if search_info.dateyyyyww.is_empty() {
        folder_path = format!(
            "{}\\{}\\{}",
            search_info.drive_letter, search_info.folder_location, search_info.productnumber
        )
    } else {
        folder_path = format!(
            "{}\\{}\\{}\\{}\\{}",
            search_info.drive_letter,
            search_info.folder_location,
            search_info.productnumber,
            search_info.dateyyyyww,
            search_info.test_env
        )
    }

    //Make sure to save after we've written new data
    if let Err(err) = search_info.save() {
        eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
    }

    // dbg!(&folder_path);

    if let Some(file_path) = functions::search_serial_number_in_folder(&search_info) {
        dbg!("Found serial number at: {}", file_path);
    } else {
        dbg!("Serial number not found in AET or PTF folders.");
    }

    let get_log_file_path = functions::find_logfiles_paths(folder_path, search_info.clone());
    match get_log_file_path {
        Ok(paths) => {
            if paths.is_empty() {
                // Handle the case where no matches are found.
                // You can either leave it empty or handle it as needed.
            } else {
                for path in paths {
                    dbg!(&path);
                    let extracted_datetime = functions::extract_datetime(&path);
                    let extracted_ptf_eat = functions::get_ptf_aet(&path);
                    let mut _json_data: Value = json!({
                        "datetime": extracted_datetime,
                        "testenv": extracted_ptf_eat,
                        "location": path.to_string(),
                        "serialnumber": search_info.serialnumber,
                    });

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
        _ => println!("{}", "No matches found"),
    }

    // dbg!(&results_from_search_json);

    results_from_search_json
}

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
                            //TODO: implement structinformation.get_config_location = true;
                            match confy::get_configuration_file_path("find_testlog", None) {
                                Ok(file) => {
                                    println!(
                                        "{} {:#?}",
                                        "Configuration file is located at:".green().bold(),
                                        file
                                    );
                                }
                                Err(err) => {
                                    eprintln!("Failed to get configuration file path: {}", err);
                                }
                            };
                            exit(2);
                        }
                        _ => functions::not_implemented(app.handle()),
                    }
                }
            }

            if cli_enabled {
                let folder_path;

                if search_info.dateyyyyww.is_empty() {
                    folder_path = format!(
                        "{}\\{}\\{}",
                        search_info.drive_letter,
                        search_info.folder_location,
                        search_info.productnumber
                    )
                } else {
                    folder_path = format!(
                        "{}\\{}\\{}\\{}\\{}",
                        search_info.drive_letter,
                        search_info.folder_location,
                        search_info.productnumber,
                        search_info.dateyyyyww,
                        search_info.test_env
                    )
                }

                //Make sure to save after we've written new data
                if let Err(err) = search_info.save() {
                    eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
                }

                if search_info.serialnumber.is_empty() && cli_enabled {
                    eprintln!("{}", "SN cannot be empty".red().bold());
                    exit(2);
                }

                let get_log_file_path =
                    functions::find_logfiles_paths(folder_path, search_info.clone());
                match get_log_file_path {
                    Ok(paths) => {
                        if paths.is_empty() {
                            println!("{}", "No matches found".red().bold());
                        } else {
                            println!("{}", "Matched log file paths:".green().bold());
                            for path in paths {
                                println!("{}", path);
                            }
                        }
                    }
                    Err(err) => eprintln!("{} {}", "Error:".red().bold(), err),
                }
                
                exit(0);
            }


            Ok(())
        })
        .invoke_handler(tauri::generate_handler![parse_frontend_search_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn cli_gui(app: tauri::AppHandle) -> Result<(), tauri::Error> {
    debug!("showing gui");
    //functions::remove_windows_console();
    tauri::WindowBuilder::new(
        &app,
        "FindTestlog",
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("Find Testlog")
    .inner_size(800., 480.)
    .resizable(true)
    .build()?;
    debug!("this won't show on Windows release builds");
    Ok(())
}
