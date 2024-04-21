use regex::Regex;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

/// input: crate::struct::Appconfig
///
/// output:
///
/// OK() -> folderpath to file ->
///
/// D\:TestLogs\6107-2100-6501\2002-W27\PI\20231006_194703_CLNT7942_group_0_39-69-G0E-4QA.log
///
/// Err() -> error message
pub fn search_for_log(search_info: &crate::structs::AppConfig) -> Result<Vec<String>, io::Error> {
    //Parse user input data to uppercase. Not for folderlocation because its doesn't follow a standard.
    let product_number: &String = &search_info.productnumber.clone().to_uppercase();
    let serial_number: &String = &search_info.serialnumber.clone().to_ascii_uppercase();
    let date_yyyyww: &String = &search_info.dateyyyyww.clone().to_uppercase();
    let drive_letter: &String = &search_info.drive_letter;
    let folder_location: &String = &search_info.folder_location;
    let test_suite: &String = &search_info.test_suite.clone().to_ascii_uppercase();

    // Create the folder path to search.
    let folder_path: PathBuf = [&drive_letter, &folder_location, &product_number]
        .iter()
        .collect::<PathBuf>();

    // Create a regular expression to match the log file names.
    let log_pattern = format!(".*{}.*", serial_number);

    // Create a vector to store the log file paths.
    let mut log_file_paths: Vec<String> = Vec::new();
    let mut found_match = false;

    let log_re = match Regex::new(&log_pattern) {
        Ok(regex) => regex,
        Err(_) => {
            log::error!("Invalid Regex: {}", log_pattern);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Regex"));
        }
    };

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
        log::info!("log file paths: {:?}", log_file_paths);
        Ok(log_file_paths)
    } else {
        log::error!("Could not find log file");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find log file",
        ));
    }
}

/// Checks if the given value is within date range parsed from app
fn is_in_date_range(entry: &DirEntry, date: &String) -> bool {
    if date.is_empty() {
        return true;
    }

    // Split the path into components.
    let path: &Path = entry.path();
    let components: Vec<_> = path.components().collect();

    /*
    Check if any of the path components contain the date string.

    components splits string up into parts, so /path/to/file.txt is split into ["path", "to", "file.txt]
    any check if any of the components contains the date string.

     */
    for comp in components {
        match comp.as_os_str().to_str() {
            Some(s) if s.contains(date) => return true,
            None => {
                log::error!("Unable to check for date range: {}", date);
                return false;
            }
            _ => (),
        }
    }

    false
}

/// Checks if the given value matches string from app config
fn is_in_test_suite(entry: &DirEntry, test_env: &String) -> bool {
    if test_env.is_empty() {
        return true;
    }

    // Split the path into components.
    let path: &Path = entry.path();
    let components: Vec<_> = path.components().collect();

    /*
    Check if any of the path components contain the date string.

    components splits string up into parts, so /path/to/file.txt is split into ["path", "to", "file.txt]
    any check if any of the components contains the date string.

     */
    for comp in components {
        match comp.as_os_str().to_str() {
            Some(s) if s.contains(test_env) => return true,
            None => {
                log::error!("Unable to check for test suite; {}", test_env);
                return false;
            }
            _ => (),
        }
    }

    false
}
