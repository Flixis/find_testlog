use colored::Colorize;
use log::warn;
use regex::Regex;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::io;
use tauri::api::cli::ArgData;
use walkdir::WalkDir;



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
pub fn remove_windows_console() {
    unsafe {
        windows_sys::Win32::System::Console::FreeConsole();
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
    let re = Regex::new(r"(\d{8})_(\d{6})").unwrap();
    let caps = re.captures(log_path).unwrap();

    let date_str = caps[1].to_string();
    let time_str = caps[2].to_string();

    // Parse date and time strings into chrono objects
    let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d").unwrap();
    let time = NaiveTime::parse_from_str(&time_str, "%H%M%S").unwrap();

    // Create a combined datetime object
    let datetime = NaiveDateTime::new(date, time);

    // Format the datetime object into the desired format
    let formatted_datetime = datetime.format("%Y/%m/%d %H:%M:%S").to_string();

    formatted_datetime
}


/*

Regex pattern matches on PTF or AET in string.
Used for confirming whether the returned path is actually correctly pulled from source directory.

*/
pub fn get_ptf_aet(input_string: &str) -> String {
    let re = Regex::new(r"(PTF|AET)").unwrap();
    let ptf_aet = re.find(input_string);

    match ptf_aet {
        Some(m) => m.as_str().to_string(),
        None => "".to_string(),
    }
}

/*

Logic of the application.
Please refractor me
TODO: write new and better version. -> Hint: see fn below.

*/
pub fn find_logfiles_paths(
    folder_path: String,
    search_info_struct: crate::structs::AppConfig,
) -> Result<Vec<String>, io::Error> {
    // Keep track of whether a match is found
    let mut found_match: bool = false;
    let mut log_file_paths: Vec<String> = Vec::new();

    // Iterate over the files in the folder path
    for entry in WalkDir::new(folder_path) {
        if let Ok(entry) = entry {
            let file_name: String = entry.file_name().to_string_lossy().to_lowercase();
            let sn_lower: String = search_info_struct
                .serialnumber
                .clone()
                .to_string()
                .to_ascii_lowercase();

            // Check if the file name contains the serial number
            if file_name.contains(&sn_lower) {
                found_match = true;
                // dbg!("{}", entry.path().display());
                if search_info_struct.open_log {
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

//working on new version of itterator
#[warn(unused)]
pub fn search_serial_number_in_folder(search_info: &crate::structs::AppConfig) -> Option<String> {
    let base_path = if search_info.dateyyyyww.is_empty() {
        format!(
            "{}\\{}\\{}",
            search_info.drive_letter, search_info.folder_location, search_info.productnumber
        )
    } else {
        format!(
            "{}\\{}\\{}\\{}\\{}",
            search_info.drive_letter,
            search_info.folder_location,
            search_info.productnumber,
            search_info.dateyyyyww,
            search_info.test_env
        )
    };

    // Try to search in the PTF folder
    let ptf_path = format!("{}/PTF", base_path);
    let ptf_file_path = format!("{}\\{}.log", ptf_path, search_info.serialnumber);

    if std::fs::metadata(&ptf_file_path).is_ok() {
        return Some(ptf_file_path);
    }

    // Try to search in the AET folder
    let aet_path = format!("{}/AET", base_path);
    let aet_file_path = format!("{}\\{}.log", aet_path, search_info.serialnumber);

    if std::fs::metadata(&aet_file_path).is_ok() {
        return Some(aet_file_path);
    }

    None
}
