use colored::*;

/// Setup for the main app window.
/// 
/// Can be resized.
/// 
/// Uses 'index.html'.
/// 
/// Note: Windows as per usual requires extra settings, so thats done here aswell.
pub fn main_window(app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
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

/// Error dialog setup.
/// 
/// Cannot be resized.
/// 
/// Uses 'error_dialog.html'.
pub fn error_dialog(app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    tauri::WindowBuilder::new(
        &app,
        "Error",
        tauri::WindowUrl::App("error_dialog.html".into()),
    )
    .title("Find Testlog")
    .inner_size(507., 284.) //exact number for aesthetic reasons
    .resizable(false)
    .build()?;
    #[cfg(all(not(debug_assertions), windows))]
    windows_helpers::hide_windows_console(true); //<--- this function should be take a bool, I want the user to be able to see the CLI if they desire.
    Ok(())
}
