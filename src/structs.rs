use clap::Parser;
use confy::ConfyError;

// Struct for application configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub drive_letter: String,
    pub folder_location: String,
    pub pn: String,
    pub test_env: String,
    pub sn: String,
}

// Default values for AppConfig
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            drive_letter: String::from("Q:"),
            folder_location: String::from("TestLogs"),
            pn: String::from(""),
            test_env: String::from("PTF"),
            sn: String::from(""),
        }
    }
}

// Methods for AppConfig
impl AppConfig {
    // Load configuration from file
    pub fn load() -> Result<Self, ConfyError> {
        confy::load("find_testlog", None)
    }

    // Save configuration to file
    pub fn save(&self) -> Result<(), ConfyError> {
        confy::store("find_testlog", None, self)
    }

    // Load configuration or use default values
    pub fn default_values() -> AppConfig {
        Self::load().unwrap_or_default()
    }
}

// Struct for command line arguments
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    ///Drive letter, Example: D:.
    pub drive_letter: Option<String>,

    #[clap(short, long)]
    ///Folder location, Example: TestLogs.
    pub folder_location: Option<String>,

    #[clap(short, long)]
    ///Product Number, Example: 6107-2100-6301.
    pub pn: Option<String>,

    #[clap(short, long)]
    ///Year Week, Example: 2023-W51, defaults to latest year-week.
    pub year_week: Option<String>,

    #[clap(short, long)]
    ///Test environment, Example: PTF
    pub test_env: Option<String>,

    #[clap(short, long)]
    ///Serial Number, Example: 22-39-A2Y-15I
    pub sn: Option<String>,

    #[clap(short, long)]
    ///If passed, Returns config location
    pub get_config_location: bool,
    
    #[clap(short, long)]
    ///Will automatically open the resulting log files, WARNING OPENS ALL OF THEM.
    pub open_log: bool,
}
