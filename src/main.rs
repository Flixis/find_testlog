use clap::Parser;
use walkdir::WalkDir;
use confy::ConfyError;
use colored::*;
use std::fs;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub drive_letter: String,
    pub folder_location: String,
    pub pn: String,
    pub test_env: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            drive_letter: String::from("D:"),
            folder_location: String::from("TestLogs"),
            pn: String::from("6107-2100-6301"),
            test_env: String::from("PTF"),
        }
    }
}

impl AppConfig {
    fn load() -> Result<Self, ConfyError> {
        confy::load("find_testlog", None)
    }

    fn save(&self) -> Result<(), ConfyError> {
        confy::store("find_testlog", None, self)
    }

    fn default_values() -> AppConfig {
        Self::load().unwrap_or_default()
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    drive_letter: Option<String>,

    #[clap(short, long)]
    folder_location: Option<String>,

    #[clap(short, long)]
    pn: Option<String>,

    #[clap(short, long)]
    week_year: Option<String>,

    #[clap(short, long)]
    test_env: Option<String>,

    #[clap(short, long)]
    sn: Option<String>,

    #[clap(short, long)]
    get_config_location: bool,
}

fn main() {
    let default_app_config = AppConfig::default_values();
    let args = Cli::parse();

    if args.get_config_location {
        let file = confy::get_configuration_file_path("find_testlog", None).unwrap();
        println!("{} {:#?}", "Configuration file is located at:".red().bold(), file);
        return;
    }


    let drive_letter = args.drive_letter.unwrap_or_else(|| default_app_config.drive_letter);
    let folder_location = args.folder_location.unwrap_or_else(|| default_app_config.folder_location);
    let pn = args.pn.unwrap_or_else(|| default_app_config.pn);
    let test_env = args.test_env.unwrap_or_else(|| default_app_config.test_env);

    // Build the folder path for the default latest search
    let folder_path = format!(
        "{}\\{}\\{}\\",
        drive_letter, folder_location, pn
    );
        
    let latest_week_year = get_most_recent_folder_name(&folder_path);

    let week_year = args.week_year.unwrap_or_else(|| latest_week_year); // Provide a default value
    let sn = args.sn.unwrap_or_else(|| String::from("22-39-A2Y-16A")); // Provide a default value

    // Build the folder path
    let folder_path = format!(
        "{}\\{}\\{}\\{}\\{}",
        drive_letter, folder_location, pn, week_year, test_env
    );

    // Save the configuration
    let app_config = AppConfig {
        drive_letter,
        folder_location,
        pn,
        test_env,
    };
    if let Err(err) = app_config.save() {
        eprintln!("Failed to save configuration: {}", err);
    }

    // Iterate over the files in the folder path
    for entry in WalkDir::new(folder_path) {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            let sn_lower = sn.to_lowercase();

            // Check if the file name contains the serial number
            if file_name.contains(&sn_lower) {
                println!("{}", entry.path().display());
            }
        }
    }
}


fn get_most_recent_folder_name(path: &str) -> String {
    let folder_names = fs::read_dir(path)
        .ok()
        .unwrap_or_else(|| {
            eprintln!("Failed to read directory: {}", path);
            std::fs::read_dir(".").unwrap() // Empty ReadDir iterator
        })
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            let folder_name = file_name.to_string_lossy().to_string();
            Some(folder_name)
        })
        .filter(|folder_name| {
            folder_name.len() >= 7 && folder_name[..4].parse::<i32>().is_ok()
        })
        .collect::<Vec<String>>();

    let most_recent_folder = folder_names.into_iter().max_by_key(|folder| {
        let year = folder[..4].parse::<i32>().unwrap();
        let week = folder[6..].parse::<i32>().unwrap();
        (year, week)
    });

    most_recent_folder.unwrap_or_else(|| {
        eprintln!("No matching folders found.");
        String::new()
    })
}