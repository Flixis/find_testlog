use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufRead;
use walkdir::{DirEntry, WalkDir};

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
            dbg!(&datetime);

            // Format the datetime object into the desired format
            let formatted_datetime = datetime.format("%Y/%m/%d %H:%M:%S").to_string();

            dbg!(&formatted_datetime);
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

Regex pattern matches on the '\test_env\' | \PTF\ | \AET\ in string.
Used for confirming whether the returned path is actually correctly pulled from source directory.

*/
pub fn extract_info_from_log(log_path_file: &str) -> Option<(String, u32, String)> {
    // Open the file for reading
    if let Ok(file) = File::open(log_path_file) {
        // Create a regular expression pattern to match the desired text
        let re = Regex::new(r"Operation configuration: (\w+(?: \w+)*).*?id: (\d+); Release (\w+)").unwrap();

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

    // Return a default value when no matches are found
    Some((
        "Couldn't determine testtype".to_string(),
        0, // no match so return default 0
        "Couldn't determine release".to_string(),
    ))
}



