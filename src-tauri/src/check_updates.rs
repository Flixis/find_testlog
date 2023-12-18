use crate::structs;
// use dotenv::dotenv;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION"); //<-- read from cargo.toml

/// Returns err if it cannot find mode_key_pass env string
/// 
/// Returns err if versioning from server is 0.0.0 or 9.9.9
/// 
/// Returns err if cannot check versioning from server
/// 
/// Bypassable with mode_key_pass env string by matching it app config
pub async fn check_for_updates(
    app: tauri::AppHandle,
    search_info: structs::AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    // dotenv().ok();

    // Initialize the updater
    let updater = tauri::updater::builder(app.clone());

    let mode_key_pass = match env::var("MODE_KEY_PASS") {
        Ok(value) => value,
        Err(e) => {
            log::error!("Error reading 'mode_key_pass' environment variable: {}", e);
            String::new() //return empty string
        }
    };

    if search_info.mode_key != mode_key_pass {
        // Perform the update check
        let update = match updater.check().await {
            Ok(update) => update,
            Err(err) => {
                return Err(err.into());
            }
        };

        log::info!("Version from server: {}", update.latest_version());

        if ["0.0.0", "9.9.9"].contains(&update.latest_version()) {
            return Err("Something went wrong with the updater".into());
        } else {
            log::info!("continuing with: {}", update.current_version());
            Ok(())
        }
    } else {
        log::warn!("Bypassing updater, continuing with: {}", VERSION);
        Ok(())
    }
}
