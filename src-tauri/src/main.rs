// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use colored::*;
use indexmap::IndexMap;

mod cli;
mod extractors;
mod search;
mod structs;
mod windows_helpers;

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
        eprintln!(
            "{}",
            "WARNING CLI WILL NOT RECEIVE UPDATES PAST V2.4.0"
                .red()
                .bold()
        );
        let search_info = cli::parse_cli_args(commandlinearguments);
        cli::execute_search_results_from_cli(search_info); //<-- this should be called seperatly in the main thread.... but for simplicity its here.
        eprintln!(
            "{}",
            "WARNING CLI WILL NOT RECEIVE UPDATES PAST V2.4.0"
                .red()
                .bold()
        );
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
) -> Vec<IndexMap<String, String>> {
    let mut search_info = structs::AppConfig::default_values();

    search_info.serialnumber = serialnumber.expect("serial number must be provided");
    search_info.productnumber = productnumber.expect("product number must be provided");
    search_info.dateyyyyww = dateyyyyww.expect("dateyyyyww must be provided");
    search_info.test_type = testtype.expect("test type must be provided");

    let mut result_data = Vec::new(); // Create an empty Vec to store multiple items

    dbg!(&search_info);

    // Make sure to save after we've written new data
    if let Err(err) = search_info.save() {
        eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
    }

    let log_file_path = search::search_for_log(&search_info);
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
                    let (extracted_datetime, extracted_clnt) =
                        extractors::extract_datetime_clnt_from_logpath(&path);
                    let mut data = IndexMap::new();
                    data.insert("datetime".to_string(), extracted_datetime.to_string());
                    data.insert("clnt".to_string(), extracted_clnt.to_string());
                    data.insert("location".to_string(), path.to_string());
                    let log_info = extractors::extract_info_from_log(&path);
                    if let Some(log_data) = log_info {
                        // Merge the log_data into the data IndexMap
                        data.extend(log_data);
                        result_data.push(data); // Add the data for this item to the result
                    } else {
                        eprintln!(
                            "{} {:?}",
                            "Failed to serialize to JSON".red().bold(),
                            search_info
                        )
                    }
                }
            }
        }
        _ => eprintln!("{} {:?}", "No matches found: ".red().bold(), search_info),
    }

    result_data // Return the Vec of data
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
            return std::path::PathBuf::new();
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
    windows_helpers::hide_windows_console(true); //<--- this function should be take a bool, I want the user to be able to see the CLI if they desire.
    Ok(())
}
