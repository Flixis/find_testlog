use clap::Parser;
use confy::ConfyError;

// Struct for application configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub pn: String,
    pub sn: String,
    pub year_week: String,
    pub drive_letter: String,
    pub folder_location: String,
    pub test_env: String,
}

// Default values for AppConfig
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            pn: String::from(""),
            sn: String::from(""),
            year_week: String::from(""),
            drive_letter: String::from("Q:"),
            folder_location: String::from("TestLogs"),
            test_env: String::from("PTF"),
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
    ///Product Number, Example: 6107-2100-6301.
    pub pn: Option<String>,

    #[clap(short, long)]
    ///Serial Number, Example: 22-39-A2Y-15I
    pub sn: Option<String>,

    #[clap(short, long)]
    ///Year Week, Example: 2023-W51, Defaults: Searches all year-week folders.
    pub year_week: Option<String>,

    #[clap(short, long)]
    ///Test environment, Default: PTF
    pub test_env: Option<String>,

    #[clap(short, long)]
    ///Will automatically open the resulting log files, WARNING OPENS ALL OF THEM.
    pub open_log: bool,

    #[clap(short, long)]
    ///Drive letter, Default Q:
    pub drive_letter: Option<String>,

    #[clap(short, long)]
    ///Folder location, Default: TestLogs.
    pub folder_location: Option<String>,

    #[clap(short, long)]
    ///If passed, Returns config location
    pub get_config_location: bool,
}
