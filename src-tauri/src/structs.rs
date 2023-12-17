use confy::ConfyError;

/// App configuration struct
/// These values are W/R values for the entire app
/// User can change these with config file
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub productnumber: String,
    pub serialnumber: String,
    pub dateyyyyww: String,
    pub test_type: String,
    pub drive_letter: String,
    pub folder_location: String,
    pub test_suite: String,
    pub mode_key: String,
}

/// Default values for AppConfig
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            productnumber: String::from(""),
            serialnumber: String::from(""),
            dateyyyyww: String::from(""),
            test_type: String::from(""),
            drive_letter: String::from("Q:"),
            folder_location: String::from("TestLogs"),
            test_suite: String::from(""),
            mode_key: String::from(""),
        }
    }
}

/// Methods for AppConfig
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
