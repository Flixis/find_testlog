use dotenv::dotenv;
use std::env;
use crate::structs;

/// Returns err if it cannot find mode_key_pass env string
/// Returns err if versioning from server is 0.0.0 or 9.9.9
/// Returns err if cannot check versioning from server
/// Bypassable with mode_key_pass env string by matching it app config
pub async fn check_for_updates(
    app: tauri::AppHandle,
    search_info: structs::AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Initialize the updater
    let updater = tauri::updater::builder(app.clone());

    // Perform the update check
    let update = match updater.check().await {
        Ok(update) => update,
        Err(err) => {
            log::error!("Failed to get update information: {}", err);
            return Err(err.into());
        }
    };

    let mode_key_pass = match env::var("mode_key_pass") {
        Ok(value) => value,
        Err(e) => {
            log::error!("Error reading 'mode_key_pass' environment variable: {}", e);
            String::new() //return empty string
        }
    };

    log::info!("Version from server: {}", update.latest_version());

    if search_info.mode_key != mode_key_pass{
        if ["0.0.0", "9.9.9"].contains(&update.latest_version()){
            log::error!("Something went wrong with the updater");
            return Err("Updater error".into());
        } else {
            log::info!(
                "continuing with: {}",
                update.current_version()
            );
            Ok(())
        }
    } else {
        log::warn!("Bypassing updater, continuing with: {}", update.current_version());
        Ok(())
    }
}
