use colored::Colorize;
use log::warn;
use regex::Regex;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::io;
use std::path::Path;
use tauri::api::cli::ArgData;
use walkdir::{WalkDir, DirEntry};

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
        if switch{
            windows_sys::Win32::System::Console::FreeConsole();
        }else {
            windows_sys::Win32::System::Console::AllocConsole();            
        }
    }
}

pub fn strip_string_of_leading_and_trailing_slashes(unescaped_string: ArgData) -> String {
    if let Some(unescaped_string) = unescaped_string.value.as_str() {
        unescaped_string.replace("\\n", "\n").replace("\\t", "\t")
    } else {
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
        },
        None => {
            // Handle the case where the regex does not match.
            log::error!("Could not extract datetime from log path: {}", log_path);
            String::new()
        },
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
        },
        None => {
            // Handle the case where the regex does not match.
            log::error!("Could not find test_env string in test environment string: {}", test_environment_string);
            "Could not find test_env string".to_string()
        },
    }
}

/*


input: crate::struct::Appconfig
output: 
OK() -> folderpath to file -> 
    D\:TestLogs\6107-2100-6501\2002-W27\PI\20231006_194703_CLNT7942_group_0_39-69-G0E-4QA.log
Err() -> error message

*/

pub fn search_for_log(search_info: &crate::structs::AppConfig) {
    let productnumber: &String = &search_info.productnumber;
    let serialnumber: &String = &search_info.serialnumber;
    let dateyyyyww: &String = &search_info.dateyyyyww;
    let driveletter: &String = &search_info.drive_letter;
    let folderlocation: &String = &search_info.folder_location;
    let test_env: &String = &search_info.test_env;
    let open_log: &bool = &search_info.open_log; //unused here, should be handled elsewhere.

    let folder_path = format!("{}\\{}\\{}", driveletter, folderlocation, productnumber);
    let log_pattern = format!(".*{}.*", serialnumber);

    let log_re = Regex::new(&log_pattern).unwrap();

    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if let Some(file_name) = entry.file_name().to_str() {
            if log_re.is_match(file_name) && is_in_date_range(&entry, dateyyyyww) && is_in_test_env(&entry, test_env) {
                println!("Found: {:?}", entry.path());
                // Here you can add logic to open the log if open_log is true
            }
        }
    }
}

fn is_in_date_range(entry: &DirEntry, date: &String) -> bool {
    if date.is_empty() {
        return true;
    }
    let path: &Path = entry.path();
    let components: Vec<_> = path.components().collect();
    components.iter().any(|comp| comp.as_os_str().to_str().unwrap().contains(date))
}

fn is_in_test_env(entry: &DirEntry, test_env: &String) -> bool {
    if test_env.is_empty() {
        return true;
    }
    let path: &Path = entry.path();
    let components: Vec<_> = path.components().collect();
    components.iter().any(|comp| comp.as_os_str().to_str().unwrap().contains(test_env))
}