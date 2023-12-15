/*

Create a GUI with following options.

*/
pub fn generic_error_dialog(app: tauri::AppHandle) -> Result<(), tauri::Error> {
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