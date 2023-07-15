use clap::Parser;
use walkdir::WalkDir;
use confy::ConfyError;
use colored::*;
use std::fs;
use std::process;

/*
Written 15/07/2023
Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.

*/

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub drive_letter: String,
    pub folder_location: String,
    pub pn: String,
    pub test_env: String,
    pub sn: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            drive_letter: String::from("D:"),
            folder_location: String::from("TestLogs"),
            pn: String::from(""),
            test_env: String::from("PTF"),
            sn: String::from(""),
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
    ///Drive letter, Example: D:.
    drive_letter: Option<String>,

    #[clap(short, long)]
    ///Folder location, Example: TestLogs.
    folder_location: Option<String>,

    #[clap(short, long)]
    ///Product Number, Example: 6107-2100-6301.
    pn: Option<String>,

    #[clap(short, long)]
    ///Year Week, Example: 2023-W51, defaults to latest year-week.
    year_week: Option<String>,

    #[clap(short, long)]
    ///Test environment, Example: PTF
    test_env: Option<String>,

    #[clap(short, long)]
    ///Serial Number, Example: 22-39-A2Y-15I
    sn: Option<String>,

    #[clap(short, long)]
    ///If passed, Returns config location
    get_config_location: bool,
    
    #[clap(short, long)]
    ///Will automatically open the resulting log files, WARNING OPENS ALL OF THEM.
    open_log: bool,
}

fn main() {
    let default_app_config = AppConfig::default_values();
    let args = Cli::parse();

    //Returns the config location
    if args.get_config_location {
        let file = confy::get_configuration_file_path("find_testlog", None).unwrap();
        println!("{} {:#?}", "Configuration file is located at:".red().bold(), file);
        return;
    }

    let drive_letter = args.drive_letter.unwrap_or_else(|| default_app_config.drive_letter);
    let folder_location = args.folder_location.unwrap_or_else(|| default_app_config.folder_location);
    let pn = args.pn.unwrap_or_else(|| default_app_config.pn);
    let test_env = args.test_env.unwrap_or_else(|| default_app_config.test_env);

    // Build the folder path, used for get_most_recent_folder_name
    let folder_path = format!(
        "{}\\{}\\{}\\",
        drive_letter, folder_location, pn
    );
    let latest_year_week = get_most_recent_folder_name(&folder_path);
    let year_week = args.year_week.unwrap_or_else(|| latest_year_week);

    let sn = args.sn.clone().unwrap_or_else(|| default_app_config.sn);
    
    if sn.is_empty(){
        eprintln!("{}", "SN cannot be empty".red().bold());
        process::exit(1);
    }

    // Build the folder path, this time with all of its values to parse for finding the log file.
    let folder_path = format!(
        "{}\\{}\\{}\\{}\\{}",
        drive_letter, folder_location, pn, year_week, test_env
    );

    let sn_clone = sn.clone(); //This workaround is so dumb, but I couldn't think of a better way to get around the borrow checking.
    let app_config = AppConfig {
        drive_letter,
        folder_location,
        pn,
        test_env,
        sn: sn_clone,
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
            if file_name.contains(&sn_lower) && args.open_log{
                match open::that(entry.path()) {
                    Ok(()) => println!("{} {}", "Opening Succefully.".green().bold(), entry.path().display()),
                    Err(err) => panic!("{} {} {}", "An error occurred when opening".red().bold(), entry.path().display(),err),
                }
            }
        }else {
            eprintln!("{}","Something went wrong (Folder likely doesn't exist)".red().bold());
        }
    }
}


fn get_most_recent_folder_name(path: &str) -> String {
    //Start by reading all the folders inside the given path
    let folder_names = fs::read_dir(path)
        .ok()
        .unwrap_or_else(|| {
            eprintln!("Failed to read directory: {}", path);
            std::fs::read_dir(".").unwrap() // Empty ReadDir iterator
        })
        .filter_map(|entry| { //build the filter for finding the folders
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            let folder_name = file_name.to_string_lossy().to_string();
            Some(folder_name)
        })
        .filter(|folder_name| { //now check if the foldername has atleast 7chars or more, if not, its not relevant.
            folder_name.len() >= 7 && folder_name[..4].parse::<i32>().is_ok() //convert the first 4 into a INT32, because YYYY format.
        })
        .collect::<Vec<String>>();

    //Now we filter again, but this time we return the highest value folder.
    let most_recent_folder = folder_names.into_iter().max_by_key(|folder| {
        let year = folder[..4].parse::<i32>().unwrap(); //Check the YYYY
        let week = folder[6..].parse::<i32>().unwrap(); //Chcek the WW
        (year, week)
    });

    most_recent_folder.unwrap_or_else(|| {
        eprintln!("No matching folders found.");
        String::new()
    })
}