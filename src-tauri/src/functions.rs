use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use log::warn;
use regex::Regex;
use std::io;
use std::path::Path;
use tauri::api::cli::ArgData;
use walkdir::{DirEntry, WalkDir};
use colored::*;

/*

Called when -h is parsed...
I should really figure out if I can just get the 'clap' -h window to do this...

Because i'm not using Tauri in its intended way, this is not really easily done.

*/
pub fn not_implemented(app: tauri::AppHandle) {
    println!("{:?}", app.package_info());
    warn!("Function not implemented yet");
    println!("Function not implemented yet");
    app.exit(127);
}

/*
required to removed windows console when launching GUI.
Tauri by default does not support this feature.

*/
pub fn hide_windows_console(switch: bool) {
    unsafe {
        if switch {
            windows_sys::Win32::System::Console::FreeConsole();
        } else {
            windows_sys::Win32::System::Console::AllocConsole();
        }
    }
}

pub fn strip_string_of_leading_and_trailing_slashes(unescaped_string: ArgData) -> String {
    // Check if the ArgData value has a string value.
    if let Some(unescaped_string) = unescaped_string.value.as_str() {
        // Replace all instances of the \n and \t escape sequences with the actual newline and tab characters, respectively.
        unescaped_string.replace("\\n", "\n").replace("\\t", "\t")
    } else {
        // If the ArgData value does not have a string value, return an empty string.
        return "".to_string();
    }
}

/*

Regex pattern matches on date and time.
then gets converted to string.

time and date required to build valid date time string.

*/
pub fn extract_datetime(log_path: &str) -> String {
    let re = Regex::new(r"(\d{8}).(\d{6})").unwrap();
    let regex_captures = re.captures(log_path);
    // dbg!(&regex_captures);
    match regex_captures {
        Some(captures) => {
            let date_str = captures[1].to_string();
            let time_str = captures[2].to_string();

            // Parse date and time strings into chrono objects
            let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d").unwrap();
            let time = NaiveTime::parse_from_str(&time_str, "%H%M%S").unwrap();

            // Create a combined datetime object
            let datetime = NaiveDateTime::new(date, time);

            // Format the datetime object into the desired format
            let formatted_datetime = datetime.format("%Y/%m/%d %H:%M:%S").to_string();

            formatted_datetime
        }
        None => {
            // Handle the case where the regex does not match.
            log::error!("Could not extract datetime from log path: {}", log_path);
            String::new()
        }
    }
}

/*

Regex pattern matches on the '\test_env\' | \PTF\ | \PTF\ in string.
Used for confirming whether the returned path is actually correctly pulled from source directory.

*/
pub fn get_test_env_string(test_environment_string: &str) -> String {
    let re = Regex::new(r"\\([A-Z])[A-Z]{1,2}").unwrap();
    let regex_captures = re.captures(test_environment_string);

    match regex_captures {
        Some(captures) => {
            let mut test_environment = captures[0].to_string();

            // Remove the leading backslash from the test environment string.
            test_environment = test_environment[1..captures[0].len()].to_string();

            // Return the test environment string.
            test_environment
        }
        None => {
            // Handle the case where the regex does not match.
            log::error!(
                "Could not find test_env string in test environment string: {}",
                test_environment_string
            );
            "Could not find test_env string".to_string()
        }
    }
}

/*


input: crate::struct::Appconfig
output:
OK() -> folderpath to file ->
    D\:TestLogs\6107-2100-6501\2002-W27\PI\20231006_194703_CLNT7942_group_0_39-69-G0E-4QA.log
Err() -> error message

*/

pub fn search_for_log(search_info: &crate::structs::AppConfig) -> Result<Vec<String>, io::Error> {
    let product_number: &String = &search_info.productnumber;
    let serial_number: &String = &search_info.serialnumber;
    let date_yyyyww: &String = &search_info.dateyyyyww;
    let drive_letter: &String = &search_info.drive_letter;
    let folder_location: &String = &search_info.folder_location;
    let test_env: &String = &search_info.test_env;
    let open_log: bool = search_info.open_log;

    // Create the folder path to search.
    let folder_path = format!("{}\\{}\\{}", drive_letter, folder_location, product_number);

    // Keep track of whether a match is found
    let mut found_match: bool = false;
    let mut log_file_paths: Vec<String> = Vec::new();

    // Iterate over the files in the folder path
    for entry in WalkDir::new(folder_path) {
        if let Ok(entry) = entry {
            let file_name: String = entry.file_name().to_string_lossy().to_lowercase();
            let sn_lower: String = serial_number.as_str().to_lowercase();

            // Check if the file name contains the serial number
            if file_name.contains(&sn_lower) {
                found_match = true;
                // dbg!("{}", entry.path().display());
                if open_log {
                    match open::that(entry.path()) {
                        Ok(()) => println!(
                            "{} {}",
                            "Opening Successfully.".green().bold(),
                            entry.path().display()
                        ),
                        Err(err) => {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                format!(
                                    "An error occurred when opening {}: {}",
                                    entry.path().display(),
                                    err
                                ),
                            ))
                        }
                    }
                }
                log_file_paths.push(entry.path().display().to_string());
            }
        }
    }
    if found_match {
        return Ok(log_file_paths);
    } else {
        // If no match is found, return an error
        Err(io::Error::new(io::ErrorKind::NotFound, "No matches found"))
    }
}

fn is_in_date_range(entry: &DirEntry, date: &String) -> bool {
    // If the date string is empty, return true.
    if date.is_empty() {
        return true;
    }

    // Get the path to the file or directory.
    let path: &Path = entry.path();

    // Split the path into components.
    let components: Vec<_> = path.components().collect();

    // Check if any of the path components contain the date string.
    /*

    components splits string up into parts, so /path/to/file.txt is split into ["path", "to", "file.txt]
    any check if any of the components contains the date string.

     */
    components
        .iter()
        .any(|comp| comp.as_os_str().to_str().unwrap().contains(date))
}

fn is_in_test_env(entry: &DirEntry, test_env: &String) -> bool {
    // If the test environment string is empty, return true.
    if test_env.is_empty() {
        return true;
    }

    // Get the path to the file or directory.
    let path: &Path = entry.path();

    // Split the path into components.
    let components: Vec<_> = path.components().collect();

    // Check if any of the path components contain the test environment string.
    components
        .iter()
        .any(|comp| comp.as_os_str().to_str().unwrap().contains(test_env))
}
