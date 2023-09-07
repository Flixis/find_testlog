// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchInfo {
    pub drive_letter: String,
    pub folder_location: String,
    pub pn: String,
    pub test_env: String,
    pub sn: String,
    pub year_week: String,
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn parse_search_terms(search_terms: serde_json) -> String {
   
//     format!(
//         "Hello, {} {} {} {} {} {} from Rust!",
//         search_terms.drive_letter,
//         search_terms.folder_location,
//         search_terms.pn,
//         search_terms.test_env,
//         search_terms.sn,
//         search_terms.year_week
//     )
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
