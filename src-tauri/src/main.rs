// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use colored::*;
use log::{debug, error};
use std::process::exit;

mod functions;
mod structs;

/*
Written 07/09/2023
Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.
*/

#[tauri::command]
fn data_to_frontend(pn: Option<String>, sn: Option<String>, year_week: Option<String>, test_env: Option<String>) -> Result<String, String> {

  let mut search_info = structs::AppConfig::default_values();

  let folder_path;

  if search_info.year_week.is_empty() {
    folder_path = format!(
      "{}\\{}\\{}",
      search_info.drive_letter, search_info.folder_location, search_info.pn
    )
  } else {
    folder_path = format!(
      "{}\\{}\\{}\\{}\\{}",
      search_info.drive_letter,
      search_info.folder_location,
      search_info.pn,
      search_info.year_week,
      search_info.test_env
    )
  }

  //Make sure to save after we've written new data
  if let Err(err) = search_info.save() {
    eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
  }

  // if search_info.sn.is_empty() && cli_enabled{
  //     eprintln!("{}", "SN cannot be empty".red().bold());
  //     exit(2);
  // }

  let mut json_data;

  let get_log_file_path = functions::itter_find_log(folder_path, search_info.clone());
  match get_log_file_path {
    Ok(paths) => {
      if paths.is_empty() {
        // println!("{}", "No matches found".red().bold()); //implement error for JS
      } else {
        let mut json_data_list: Vec<String> = Vec::new();
        for path in paths {
          json_data = serde_json::to_string(&path).unwrap();
          json_data_list.push(json_data);
        }

        // Make sure the list is not empty before returning it.
        if !json_data_list.is_empty() {
          let json_data_string = serde_json::to_string(&json_data_list).unwrap();
          return Ok(json_data_string);
        }
      }
    }
    Err(err) => eprintln!("{} {}", "Error:".red().bold(), err),
  }

  // If we reach here, there was an error.
  Err("Error getting log file paths".to_string())
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
            if std::env::args_os().count() <= 1 {
                cli_enabled = false;
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
                            let saved_to_struct = functions::strip_string_of_garbage(data);
                            search_info.pn = saved_to_struct;
                        }
                        "sn" => {
                            // Create a new SearchInfo struct with only the sn field set
                            let saved_to_struct = functions::strip_string_of_garbage(data);
                            search_info.sn = saved_to_struct;
                        }
                        "year_week" => {
                            // Create a new SearchInfo struct with only the year_week field set
                            let saved_to_struct = functions::strip_string_of_garbage(data);
                            search_info.year_week = saved_to_struct;
                        }
                        "test_env" => {
                            // Create a new SearchInfo struct with only the test_env field set
                            let saved_to_struct = functions::strip_string_of_garbage(data);
                            search_info.test_env = saved_to_struct;
                        }
                        "open_log" => {
                            search_info.open_log = data.value.is_boolean();
                        }
                        "drive_letter" => {
                            // Set the drive_letter field
                            let saved_to_struct = functions::strip_string_of_garbage(data);
                            search_info.drive_letter = saved_to_struct;
                        }
                        "folder_location" => {
                            // Set the folder_location field
                            let saved_to_struct = functions::strip_string_of_garbage(data);
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
                        _ => functions::not_done(app.handle()),
                    }
                }
            }

            let folder_path;

            if search_info.year_week.is_empty() {
                folder_path = format!(
                    "{}\\{}\\{}",
                    search_info.drive_letter, search_info.folder_location, search_info.pn
                )
            } else {
                folder_path = format!(
                    "{}\\{}\\{}\\{}\\{}",
                    search_info.drive_letter,
                    search_info.folder_location,
                    search_info.pn,
                    search_info.year_week,
                    search_info.test_env
                )
            }

            //Make sure to save after we've written new data
            if let Err(err) = search_info.save() {
                eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
            }

            if search_info.sn.is_empty() && cli_enabled{
                eprintln!("{}", "SN cannot be empty".red().bold());
                exit(2);
            }

            let get_log_file_path = functions::itter_find_log(folder_path, search_info.clone());
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

            // Print the struct at the end
            dbg!("{:?}", &search_info);

            if cli_enabled{    
                exit(0);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![data_to_frontend])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn cli_gui(app: tauri::AppHandle) -> Result<(), tauri::Error> {
    debug!("showing gui");
    functions::remove_windows_console();
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


