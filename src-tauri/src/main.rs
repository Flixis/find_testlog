// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use colored::*;
use serde_json::{json, Value};
use std::process::exit;

mod cli;
mod functions;
mod structs;
mod extractors;

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
    let commandlinearguments: cli::CliCommands = cli::CliCommands::parse();

    if std::env::args_os().count() > 1 {
        eprintln!("{}", "WARNING CLI WILL NOT RECEIVE UPDATES PAST V2.4.0".red().bold());
        let search_info = cli::parse_cli_args(commandlinearguments);
        cli::execute_search_results_from_cli(search_info); //<-- this should be called seperatly in the main thread.... but for simplicity its here.
        eprintln!("{}", "WARNING CLI WILL NOT RECEIVE UPDATES PAST V2.4.0".red().bold());
    } else {
        // Builds the Tauri connection
        tauri::Builder::default()
            .setup(|app| {
                cli_gui(app.handle())?;
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                parse_frontend_search_data,
                get_configuration_file_path
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application")
    }
}


/*

Parsing data from frontend.
The search logic for the GUI part of the app is done here.


*/

#[tauri::command]
async fn parse_frontend_search_data(
    productnumber: Option<String>,
    serialnumber: Option<String>,
    dateyyyyww: Option<String>,
    testtype: Option<String>,
) -> Value {
    let mut search_info = structs::AppConfig::default_values();

    search_info.serialnumber = serialnumber.unwrap();
    search_info.productnumber = productnumber.unwrap();
    search_info.dateyyyyww = dateyyyyww.unwrap();
    search_info.test_type = testtype.unwrap();

    /*We instanciate a JSON var so we can return an empty JSON on fail */
    let mut results_from_search_json: Value = json!({
        "datetime": [],
        "testtype": [],
        "revision": [],
        "id": [],
        "clnt": [],
        "location": [],
    });

    dbg!(&search_info);

    //Make sure to save after we've written new data
    if let Err(err) = search_info.save() {
        eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
    }

    let log_file_path = functions::search_for_log(&search_info);
    // dbg!(&log_file_path);
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
                    // dbg!(&path);
                    let extracted_datetime = extractors::extract_datetime(&path);
                    let extracted_clnt = extractors::extract_clnt_string(&path);
                    let log_info = extractors::extract_info_from_log(&path);
                    if let Some((testtype, id, release)) = log_info {
                        // Handle the case when information is successfully extracted
                        let mut _json_data: Value = json!({
                            "datetime": extracted_datetime,
                            "testtype": testtype,
                            "revision": release,
                            "id": id,
                            "clnt": extracted_clnt,
                            "location": path.to_string(),
                        });
                    
                        // Push values to arrays in the JSON object
                        results_from_search_json["datetime"]
                            .as_array_mut()
                            .unwrap()
                            .push(_json_data["datetime"].take());
                        results_from_search_json["testtype"]
                            .as_array_mut()
                            .unwrap()
                            .push(_json_data["testtype"].take());
                        results_from_search_json["revision"]
                            .as_array_mut()
                            .unwrap()
                            .push(_json_data["revision"].take());
                        results_from_search_json["id"]
                            .as_array_mut()
                            .unwrap()
                            .push(_json_data["id"].take());
                        results_from_search_json["clnt"]
                            .as_array_mut()
                            .unwrap()
                            .push(_json_data["clnt"].take());
                        results_from_search_json["location"]
                            .as_array_mut()
                            .unwrap()
                            .push(_json_data["location"].take());
                    } else {
                        eprintln!("{} {:?}", "Failed to serialize to JSON".red().bold(), search_info)
                    }
                }
            }
        }
        _ => eprintln!("{} {:?}", "No matches found: ".red().bold(), search_info),
    }

    //dbg!(&results_from_search_json);
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
    println!(
        "{}",
        "Starting Test Log Finder! Tariq Dinmohamed (C)"
            .green()
            .bold()
    );
    tauri::WindowBuilder::new(
        &app,
        "FindTestlog",
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("Find Testlog")
    .inner_size(800., 480.)
    .resizable(true)
    .build()?;
    #[cfg(all(not(debug_assertions), windows))]
    functions::hide_windows_console(true); //<--- this function should be take a bool, I want the user to be able to see the CLI if they desire.
    Ok(())
}
