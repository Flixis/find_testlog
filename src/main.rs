//cargo run -- D: TestLogs 6107-2100-6301 2023-W51 PTF 22-39-A2Y-15I
use clap::{Parser};
use colored::*;
use walkdir::WalkDir;
use confy::ConfyError;


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
        confy::load("app_name", None)
    }

    fn save(&self) -> Result<(), ConfyError> {
        confy::store("app_name", None, self)
    }

    fn default_values() -> AppConfig {
        Self::load().unwrap_or_default()
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value = AppConfig::default_values)]
    drive_letter: String,

    #[clap(short, long, default_value = AppConfig::default_values)]
    folder_location: String,

    #[clap(short, long, default_value = AppConfig::default_values)]
    pn: String,

    #[clap(short, long, required = true)]
    week_year: String,

    #[clap(short, long, default_value = AppConfig::default_values)]
    test_env: String,

    #[clap(short, long, required = true)]
    sn: String,
}





fn main() {

    //Why can't I check if there are no arguments in clap?!?!
    if std::env::args().len() <= 1 {
        let file = confy::get_configuration_file_path("app_name", None).unwrap();
        println!("{} {:#?}", "Configuration file is located at:".red().bold(), file);
        
    }

    let default_app_config = AppConfig::default();
    let args = Cli::parse();
    let drive_letter = args.drive_letter.clone();
    let folder_location = args.folder_location.clone();
    let pn = args.pn.clone();
    let week_year = args.week_year.clone();
    let test_env = args.test_env.clone();
    let sn = args.sn.clone();

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
