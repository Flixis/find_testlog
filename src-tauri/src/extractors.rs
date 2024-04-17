use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use indexmap::IndexMap;
use regex::Regex;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/*

Pattern matches on datetime and clnt, which is required to build a valid date-time string.

*/
/// Extracts date and time from filepath and parses it as chrono:naivedate
pub fn extract_datetime_clnt_from_logpath(log_path: &str) -> (String, String) {
    let path = Path::new(log_path);
    let log_path = path
        .file_name()
        .and_then(|f| f.to_str())
        .ok_or_else(|| {
            log::error!("Failed to extract log filename from path");
            "Error: Could not extract log filename"
        })
        .unwrap();
    let parts: Vec<&str> = log_path.split('_').collect();

    // Ensure the parts vector has at least the expected number of elements
    if parts.len() >= 3 {
        let date_str = parts[0];
        let time_str = parts[1];
        let clnt = parts[2].to_string();

        let default_date = NaiveDate::from_ymd_opt(1, 1, 1).unwrap_or_default();
        let default_time = NaiveTime::from_hms_opt(1, 1, 1).unwrap_or_default();

        // Parse date and time strings into chrono objects
        let date = match NaiveDate::parse_from_str(date_str, "%Y%m%d") {
            Ok(date) => date,
            Err(_) => {
                log::error!("Invalid date format: {}", date_str);
                default_date
            }
        };

        let time = match NaiveTime::parse_from_str(time_str, "%H%M%S") {
            Ok(time) => time,
            Err(_) => {
                log::error!("Invalid time format: {}", time_str);
                default_time
            }
        };

        // Create a combined datetime object
        let datetime = NaiveDateTime::new(date, time);

        // Format the datetime object into the desired format
        let formatted_datetime = datetime.format("%Y/%m/%d %H:%M:%S").to_string();

        log::debug!(
            "extract_datetime_clnt_from_logpath: {} {}",
            formatted_datetime,
            clnt
        );

        (formatted_datetime, clnt)
    } else {
        log::error!(
            "Could not extract datetime or CLNT from log path: {}",
            log_path
        );
        (String::new(), String::new())
    }
}

/*

Regex pattern matches on and returns something like:

    "Name": "Q's Test Framework",
    "Version": "v3.7.8-HQHEI",
    "Machine": "CLNT5849",
    "Mode": "Development",
    "PN": "9999-1111-2222",
    "Operation": "Functional test",
    "configuration": "FT (id: 628938; Release R497 (Latest))",
    "testtype": "FT",
    "id": "628938",
    "release": "R497",

*/
/// Extracts information from log file using regex patterns:
///
/// (\w+):\s*(.+)
///
/// (\w+(?: \w+)*).*?id: (\d+); Release (\w+)
///
/// b(PASS(?:ED)?|FAIL(?:ED)?)\b
pub fn extract_info_from_log(
    filename: &str,
    bytes_to_read: i64,
) -> Result<IndexMap<String, String>, io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            log::error!("Failed to open file: {}", err);
            return Err(err.into());
        }
    };

    println!("{:?}", &file);

    let mut first_part_of_file = match read_file_till_bytes(&file, bytes_to_read) {
        Ok(content) => content,
        Err(e) => {
            log::warn!("File read operation failed: {}", e);
            String::new()
        }
    };

    let mut second_part_of_file = match read_file_till_bytes(&file, -bytes_to_read) {
        Ok(content) => content,
        Err(e) => {
            log::warn!("File read operation failed: {}", e);
            String::new()
        }
    };

    // dbg!(&first_part_of_file);
    dbg!(&second_part_of_file);

    first_part_of_file = clean_up_string(&first_part_of_file);
    second_part_of_file = clean_up_string(&second_part_of_file);

    let mut cleaned_operation_headers =
        create_header_hashmap_from_headers_string(&first_part_of_file);
    let cleaned_operation_status = create_status_hashmap_from_status_string(&second_part_of_file);

    cleaned_operation_headers.extend(cleaned_operation_status);
    Ok(cleaned_operation_headers)
}

fn read_file_till_bytes(mut file: &File, bytes_to_read: i64) -> Result<String, io::Error> {
    if bytes_to_read < 0 {
        //result unused because it returns cursor position which we dont need
        let _file_seek_result = file
            .seek(SeekFrom::End(bytes_to_read))
            .or_else(|_| file.seek(SeekFrom::End(bytes_to_read / 2)));
    }
    let mut buffer = vec![0; bytes_to_read.abs() as usize];
    let bytes_extracted_from_file = file.read(&mut buffer).unwrap();

    // Convert the buffer to a String, handling potential errors.
    let text = String::from_utf8(buffer[..bytes_extracted_from_file].to_vec())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"));

    text
}

fn clean_up_string(input: &str) -> String {
    input
        // Remove the Unicode BOM
        .replace("\u{feff}", "")
        // Convert Windows line endings to Unix line endings
        .replace("\r\n", "\n")
}

fn create_header_hashmap_from_headers_string(data: &String) -> IndexMap<String, String> {
    let mut hashmap = IndexMap::new();

    for line in data.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().replace("-", "").trim().to_lowercase(); // Format key
            let value = value.trim().to_string();

            if key == "operation configuration" {
                // Handle special formatting for "Operation configuration"
                let parts: Vec<&str> = value.split_whitespace().collect();
                if !parts.is_empty() {
                    hashmap.insert("operation_configuration".to_string(), parts[0].to_string());

                    // Further processing for id and Release parts
                    for part in &parts[1..] {
                        if part.starts_with("(id:") {
                            let id = parts[2].to_string();
                            hashmap.insert(
                                "id".to_string(),
                                id.strip_suffix(";").unwrap_or_default().to_string(),
                            );
                        } else if part.starts_with("Release") {
                            let release = parts[4..5].concat().to_string();
                            hashmap.insert("release".to_string(), release);
                            break; // Assuming rest of the parts belong to Release, stop iterating
                        }
                    }
                }
            } else {
                // For all other keys, insert directly
                hashmap.insert(key, value);
            }
        }
    }

    hashmap
}

fn create_status_hashmap_from_status_string(input: &str) -> IndexMap<String, String> {
    let mut results = IndexMap::new();

    // Find the start of the "Test Results" section
    if let Some(start) = input.find("Test Results:") {
        // Extract everything from "Test Results:" to the end of the string
        let results_section = &input[start..];

        // Iterate over each line in the "Test Results" section
        for line in results_section.lines() {
            // Check if the line contains a serial number (SN) and result (PASS/FAIL)
            if line.contains("SN:") && (line.contains("PASS") || line.contains("FAIL")) {
                // Extract the serial number and result
                let parts: Vec<&str> = line.split_whitespace().collect();
                let sn_index = parts.iter().position(|&r| r == "SN:").unwrap() + 1;
                let result_index = parts.len() - 1; // Assuming the result is always the last part

                let serial_number = parts[sn_index].trim_end_matches('-').to_string();
                let result = parts[result_index].to_string();

                // Insert the serial number and result into the HashMap
                dbg!(&parts);
                results.insert(serial_number, result);
            }
        }
    } else {
        log::warn!("Failed to parse PASS/FAIL state, using fallback");
        let pass_or_fail_regex = Regex::new(r"\b(PASS(?:ED)?|FAIL(?:ED)?)\b").unwrap();

        for line in input.lines() {
            if let Some(caps) = pass_or_fail_regex.captures(&line) {
                let key = "PASS_FAIL_STATUS".to_string();
                let value = caps[1].to_string();
                {
                    results.insert(key.clone(), value);
                }
            }
        }
    }

    results
}
