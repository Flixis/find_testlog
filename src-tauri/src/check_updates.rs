use crate::structs;
use std::env;
use sha2::{Digest, Sha256};

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

    // Initialize the updater
    let updater = tauri::updater::builder(app.clone());

    //create new hasher to hash the string from config
    let mut hasher = Sha256::new();
    hasher.update(&search_info.mode_key);
    let result_hash = hasher.finalize();
    
    // Convert both hashes to the same format for comparison
    let mode_key_pass = b"3fdf315d52676639a137ab505dd9b7eb86d456360a96f4c7ccacf1c300176f20";
    let mode_key_pass = hex::decode(mode_key_pass).expect("Invalid hex string");
    
    // Compare both hashes to confirm modekeypass is correct
    if result_hash[..] != mode_key_pass[..] {
        // Perform the update check within an async context
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
