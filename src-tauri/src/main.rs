// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::exit;

use clap::Parser;
use colored::*;
use indexmap::IndexMap;

mod cli;
mod gui;
mod extractors;
mod logging_settings;
mod search;
mod structs;
mod windows_helpers;
mod check_updates;

/*
(C) Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.
*/
fn main() {
    let commandlinearguments: cli::CliCommands = cli::CliCommands::parse();
    let search_info = structs::AppConfig::default_values();

    logging_settings::setup_loggers();

    if std::env::args_os().count() > 1 {
        log::info!("WARNING CLI WILL NOT RECEIVE UPDATES PAST V2.4.0");
        let search_info = cli::parse_cli_args(commandlinearguments);
        cli::execute_search_results_from_cli(search_info);
        log::info!("WARNING CLI WILL NOT RECEIVE UPDATES PAST V2.4.0");
    } else {
        // Builds the Tauri connection
        tauri::Builder::default()
            .setup(|app| {
                let handle = app.handle();
                tauri::async_runtime::spawn(async move {
                    let check_update = check_updates::check_for_updates(handle.clone(), search_info.clone()).await;
                    match check_update {
                        Ok(()) => {
                            let _ = gui::main_window(handle);
                        }
                        Err(err) => {
                            let _ =  gui::error_dialog(handle);
                            log::error!("ERROR: {}", err);
                        }
                    }
                });
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                parse_frontend_search_data,
                get_configuration_file_path,
                kill_app
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application")
    }
}


/// Takes values from frontend and parses through search algorithm.
/// Returns a Vec of values.
#[tauri::command]
async fn parse_frontend_search_data(
    productnumber: Option<String>,
    serialnumber: Option<String>,
    dateyyyyww: Option<String>,
    testtype: Option<String>,
) -> Vec<IndexMap<String, String>> {
    let mut search_info = structs::AppConfig::default_values();

    search_info.serialnumber = serialnumber.expect("serial number must be provided");
    search_info.productnumber = productnumber.expect("product number must be provided");
    search_info.dateyyyyww = dateyyyyww.expect("dateyyyyww must be provided");
    search_info.test_type = testtype.expect("test type must be provided");

    let mut result_data = Vec::new(); // Create an empty Vec to store multiple items

    log::info!("Search info: {:?}", search_info);

    // Make sure to save after we've written new data
    if let Err(err) = search_info.save() {
        log::error!("{} {}", "Failed to save configuration:".red().bold(), err);
    }

    let log_file_path = search::search_for_log(&search_info);
    match log_file_path {
        Ok(paths) => {
            if paths.is_empty() {
                log::error!(
                    "{} {:?}",
                    "Path could not be matched".red().bold(),
                    search_info
                );
            } else {
                for path in paths {
                    let (extracted_datetime, extracted_clnt) =
                        extractors::extract_datetime_clnt_from_logpath(&path);
                    let mut data = IndexMap::new();
                    data.insert("datetime".to_string(), extracted_datetime.to_string());
                    data.insert("clnt".to_string(), extracted_clnt.to_string());
                    data.insert("location".to_string(), path.to_string());

                    match extractors::extract_info_from_log(&path, 10) {
                        Ok(Some(log_data)) => {
                            for (key, value) in &log_data {
                                data.insert(key.clone(), value.clone());
                            }
                            result_data.push(data);
                        }
                        Ok(None) => {
                            log::error!("No data found in the 'configuration' field.");
                        }
                        Err(err) => {
                            log::error!("Error: {}", err);
                        }
                    }
                }
            }
        }
        _ => log::error!("{} {:?}", "No matches found: ".red().bold(), search_info),
    }

    result_data // Return the Vec of data
}


///Gets the path to the configuration file.
#[tauri::command]
fn get_configuration_file_path(confy_config_name: &str) -> std::path::PathBuf {
    match confy::get_configuration_file_path(&confy_config_name, None) {
        Ok(file) => {
            log::info!(
                "{} {:#?}",
                "Configuration file is located at:".green().bold(),
                file
            );
            return file;
        }
        Err(err) => {
            log::error!("Failed to get configuration file path: {}", err);
            return std::path::PathBuf::new();
        }
    };
}


/// Kills the app, used in error-dialog.
#[tauri::command]
fn kill_app(){
    exit(-1)
}