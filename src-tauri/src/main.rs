// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{error, info, LevelFilter, debug, warn};
use tauri::api::cli::ArgData;
// use clap::Parser;
// use colored::*;
// use std::env;

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


#[tauri::command] //tauri handler
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

    // tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![rust_parse_search_data])
    // .run(tauri::generate_context!())
    // .expect("error while running tauri application");

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
        };
        // Iterate over each key and execute functions based on them
        for (key, data) in matches.args {
            if data.occurrences > 0 || key.as_str() == "help" || key.as_str() == "version" {
                // Define all CLI commands/arguments here and in the tauri.conf.json file
                // WARNING: If the commmand is not defined in the tauri.conf.json file, it can't be used here
                match key.as_str() {
                    "gui" => {
                        if let Err(err) = cli_gui(app.handle()) {
                            error!("GUI Error: {}", err);
                        }
                    }
                    "pn" => testing_cli_pn(app.handle(), data),
                     _ => not_done(app.handle()),
                }
            }
        }    
        Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            /*TODO: add handlers here */
            rust_parse_search_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}    


fn cli_gui(app: tauri::AppHandle) -> Result<(), tauri::Error> {
    debug!("showing gui");
    // #[cfg(all(not(debug_assertions), windows))]
    // remove_windows_console();
    tauri::WindowBuilder::new(&app, "FindTestlog", tauri::WindowUrl::App("index.html".into()))
      .title("Find Testlog")
      .inner_size(800., 480.)
      .resizable(true)
      .build()?;
    debug!("this won't show on Windows release builds");
    Ok(())
  }

fn testing_cli_pn(app: tauri::AppHandle, data: ArgData){
    println!("hello");
    dbg!("here");
    app.exit(2);
}

fn not_done(app: tauri::AppHandle) {
    warn!("Function not implemented yet");
    app.exit(2);
}