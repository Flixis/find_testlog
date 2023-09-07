// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod structs{
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
fn parse_search_data(pn: String , sn: String, year_week: String, test_env: String) -> String {
    
    let data = structs::SearchInfo {
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_search_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
