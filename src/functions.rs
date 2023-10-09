use std::io;
use std::path::Path;
use walkdir::{WalkDir,DirEntry};
use regex::Regex;

pub fn search_for_log(search_info: &crate::structs::AppConfig) -> Result<Vec<String>, io::Error> {
    let product_number: &String = &search_info.productnumber;
    let serial_number: &String = &search_info.serialnumber;
    let date_yyyyww: &String = &search_info.dateyyyyww;
    let drive_letter: &String = &search_info.drive_letter;
    let folder_location: &String = &search_info.folder_location;
    let test_env: &String = &search_info.test_env;

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
                is_in_date_range(&entry, date_yyyyww) &&
                // Check if the file is in the test environment.
                is_in_test_env(&entry, test_env)
            {
                // Set the found_match flag to true.
                found_match = true;

                // Add the log file path to the vector.
                log_file_paths.push(entry.path().display().to_string());
            }
        }
    }

    if found_match {
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