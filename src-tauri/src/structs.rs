use confy::ConfyError;

// Struct for application configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub productnumber: String,
    pub serialnumber: String,
    pub dateyyyyww: String,
    pub drive_letter: String,
    pub folder_location: String,
    pub test_env: String,
    pub open_log: bool,
}

// Default values for AppConfig
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            productnumber: String::from(""),
            serialnumber: String::from(""),
            dateyyyyww: String::from(""),
            drive_letter: String::from("Q:"),
            folder_location: String::from("TestLogs"),
            test_env: String::from("PTF"),
            open_log: false,
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
