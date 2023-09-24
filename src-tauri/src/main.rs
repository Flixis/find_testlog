// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::exit;

use log::{debug, error, info, warn, LevelFilter};
use tauri::api::cli::get_matches;
use tauri::api::cli::ArgData;

mod functions;
mod structs;

/*
Written 07/09/2023
Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.
*/

#[tauri::command] //tauri handler
fn rust_parse_search_data(pn: String, sn: String, year_week: String, test_env: String) -> String {
    // let data = structs::AppConfig {
    //     drive_letter: "".to_string(),
    //     folder_location: "".to_string(),
    //     pn : pn,
    //     test_env: test_env,
    //     sn: sn,
    //     year_week: year_week,
    // };

    // format!("data:, {:?} from Rust!", data) <-- this gets parsed to the frontend

    exit(2)
}

fn main() {

    // Builds the Tauri connection
    tauri::Builder::default()
        .setup(|app| {
            // Default to GUI if the app was opened with no CLI args.
            if std::env::args_os().count() <= 1 {
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
            }; // Iterate over each key and execute functions based on them
            let mut structinformation = structs::AppConfig {
                drive_letter: "".to_string(),
                folder_location: "".to_string(),
                pn: "".to_string(),
                test_env: "".to_string(),
                sn: "".to_string(),
                year_week: "".to_string(),
            };
            for (key, data) in matches.args {
                if data.occurrences > 0 || key.as_str() == "help" || key.as_str() == "version" {
                    match key.as_str() {
                        "pn" => {
                            // Create a new SearchInfo struct with only the pn field set
                            structinformation.pn = data.value.clone().to_string();
                        }
                        "sn" => {
                            // Create a new SearchInfo struct with only the sn field set
                            structinformation.sn = data.value.clone().to_string();
                        }
                        _ => not_done(app.handle()),
                    }
                }
            }
            // Print the struct at the end
            println!("{:?}", structinformation);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![rust_parse_search_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn cli_gui(app: tauri::AppHandle) -> Result<(), tauri::Error> {
    debug!("showing gui");
    #[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
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

fn not_done(app: tauri::AppHandle) {
    warn!("Function not implemented yet");
    app.exit(2);
}
