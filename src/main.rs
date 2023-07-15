use clap::Parser;
use colored::*;
use std::fs;
use std::process;
use walkdir::WalkDir;

mod structs;

/*
Written 15/07/2023
Tariq Dinmohamed

I hated searching for logfiles, So I challenged myself to make something to help with that.
Documentation and code comes as is.
*/

fn main() {
    let default_app_config = structs::AppConfig::default_values();
    let args = structs::Cli::parse();

    // Returns the config location
    if args.get_config_location {
        let file = confy::get_configuration_file_path("find_testlog", None).unwrap();
        println!("{} {:#?}", "Configuration file is located at:".red().bold(), file);
        return;
    }

    // Extract arguments or use default values
    let drive_letter = args.drive_letter.as_ref().unwrap_or(&default_app_config.drive_letter).to_string();
    let folder_location = args.folder_location.as_ref().unwrap_or(&default_app_config.folder_location).to_string();
    let pn = args.pn.as_ref().unwrap_or(&default_app_config.pn).to_string();
    let test_env = args.test_env.as_ref().unwrap_or(&default_app_config.test_env).to_string();

    // Build the folder path, used for get_most_recent_folder_name
    let folder_path = format!("{}\\{}\\{}\\", drive_letter, folder_location, pn);
    let latest_year_week = get_most_recent_folder_name(&folder_path);
    let year_week = args.year_week.as_ref().unwrap_or(&latest_year_week);

    let sn = args.sn.clone().unwrap_or(default_app_config.sn);
    
    if sn.is_empty() {
        eprintln!("{}", "SN cannot be empty".red().bold());
        process::exit(1);
    }

    // Build the folder path, this time with all of its values to parse for finding the log file.
    let folder_path = format!("{}\\{}\\{}\\{}\\{}", drive_letter, folder_location, pn, year_week, test_env);

    let app_config = structs::AppConfig {
        drive_letter,
        folder_location,
        pn,
        test_env,
        sn: sn.clone(),
    };
    
    if let Err(err) = app_config.save() {
        eprintln!("Failed to save configuration: {}", err);
    }

    itter_find_log(folder_path, args.clone())
}

fn itter_find_log(folder_path: String, cli_parse: structs::Cli) {
    // Iterate over the files in the folder path  
    for entry in WalkDir::new(folder_path) {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            let sn_lower: String = cli_parse.sn.clone().unwrap_or_default().to_string().to_ascii_lowercase();

            // Check if the file name contains the serial number
            if file_name.contains(&sn_lower) {
                println!("{}", entry.path().display());
                if cli_parse.open_log {
                    match open::that(entry.path()) {
                        Ok(()) => println!("{} {}", "Opening Succefully.".green().bold(), entry.path().display()),
                        Err(err) => panic!("{} {} {}", "An error occurred when opening".red().bold(), entry.path().display(),err),
                    }
                }
            }
        } else {
            eprintln!("{}","Something went wrong (Folder likely doesn't exist)".red().bold());
        }
    }
}

fn get_most_recent_folder_name(path: &str) -> String {
    // Start by reading all the folders inside the given path
    let min_char_length_folder_name: usize = 7;
    
    let folder_names = fs::read_dir(path)
        .ok()
        .unwrap_or_else(|| {
            eprintln!("Failed to read directory: {}", path);
            fs::read_dir(".").unwrap() // Empty ReadDir iterator
        })
        .filter_map(|entry| { // Build the filter for finding the folders
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            let folder_name = file_name.to_string_lossy().to_string();
            Some(folder_name)
        })
        .filter(|folder_name| { // Now check if the foldername has at least 7 chars or more, if not, it's not relevant.
            folder_name.len() >= min_char_length_folder_name && folder_name[..4].parse::<i32>().is_ok() // Convert the first 4 chars into an INT32, because YYYY format.
        })
        .collect::<Vec<String>>();

    // Now we filter again, but this time we return the highest value folder.
    let most_recent_folder = folder_names.into_iter().max_by_key(|folder| {
        let year = folder[..4].parse::<i32>().unwrap(); // Check the YYYY
        let week = folder[6..].parse::<i32>().unwrap(); // Check the WW
        (year, week)
    });

    most_recent_folder.unwrap_or_else(|| {
        eprintln!("No matching folders found.");
        String::new()
    })
}
