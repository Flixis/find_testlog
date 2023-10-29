use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufRead;
use walkdir::{DirEntry, WalkDir};



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

Regex pattern matches on the '\test_env\' | \PTF\ | \AET\ in string.
Used for confirming whether the returned path is actually correctly pulled from source directory.

*/


pub fn extract_info_from_log(log_path: &str) -> Option<(String, u32, String)> {
    // Open the file for reading
    if let Ok(file) = File::open(log_path) {
        // Create a regular expression pattern to match the desired text
        let re = Regex::new(r"- Operation configuration: (\w+) \(id: (\d+); Release (\w+) \(Latest\)\)").unwrap();

        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                if let Some(captures) = re.captures(&line) {
                    if let (Some(testtype), Some(id), Some(release)) = (
                        captures.get(1),
                        captures.get(2),
                        captures.get(3),
                    ) {
                        return Some((
                            testtype.as_str().to_string(),
                            id.as_str().parse().unwrap(),
                            release.as_str().to_string(),
                        ));
                    }
                }
            }
        }

        eprintln!("Text not found in the file.");
    } else {
        eprintln!("Failed to open the file.");
    }

    None
}



/*

Regex pattern matches on the '\test_env\' | \PTF\ | \AET\ in string.
Used for confirming whether the returned path is actually correctly pulled from source directory.

*/
pub fn extract_clnt_string(log_path: &str) -> String {
    let re = Regex::new(r"CLNT\d+").unwrap();
    let regex_captures = re.captures(log_path);

    match regex_captures {
        Some(captures) => {
            let clnt = captures[0].to_string();
            // Return the test environment string.
            clnt
        }
        None => {
            // Handle the case where the regex does not match.
            log::error!(
                "Could not find CLNT string in test environment string: {}",
                log_path
            );
            "Could not find CLNT string".to_string()
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
    let test_suite: &String = &search_info.test_suite;

    //Parse user input data to uppercase. Not for folderlocation because its doesn't follow a standard.
    let product_number: String = product_number.to_uppercase();
    let serial_number: String = serial_number.to_uppercase();
    let date_yyyyww: String = date_yyyyww.to_uppercase();
    let drive_letter: String = drive_letter.to_uppercase();
    let test_suite: String = test_suite.to_uppercase();

    // Create the folder path to search.
    let folder_path = format!("{}\\{}\\{}", drive_letter, folder_location, product_number);

    // Create a regular expression to match the log file names.
    let log_pattern = format!(".*{}.*", serial_number);

    // Create a vector to store the log file paths.
    let mut log_file_paths: Vec<String> = Vec::new();
    let mut found_match = false;
    let log_re = Regex::new(&log_pattern).unwrap();

    // Iterate over all of the files and directories in the folder path.
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        // Get the file name.
        if let Some(file_name) = entry.file_name().to_str() {
            // Check if the file name matches the regular expression.
            if log_re.is_match(file_name) &&
                // Check if the file is in the date range.
                is_in_date_range(&entry, &date_yyyyww) &&
                // Check if the file is in the test environment.
                is_in_test_suite(&entry, &test_suite)
            {
                // Set the found_match flag to true.
                found_match = true;

                // Add the log file path to the vector.
                log_file_paths.push(entry.path().display().to_string());
            }
        }
    }

    if found_match {
        log_file_paths.reverse(); //Send the log file paths in descending order.
        Ok(log_file_paths)
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find log file.",
        ));
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

fn is_in_test_suite(entry: &DirEntry, test_env: &String) -> bool {
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
