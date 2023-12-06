use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

/*

Regex pattern matches on date and time, which is required to build a valid date-time string. Additionally, it checks for the presence of '\test_env' | \PTF\ | \AET\ in the string to confirm whether the returned path is correctly pulled from the source directory.

*/
pub fn extract_datetime_clnt_from_logpath(log_path: &str) -> (String, String) {
    let re = Regex::new(r"(\d{8}).(\d{6}).(CLNT\d+)").expect("Failed to parse log path");
    let regex_captures = re.captures(log_path);
    dbg!(&regex_captures);
    // dbg!(&regex_captures);
    match regex_captures {
        Some(captures) => {
            let date_str = captures[1].to_string();
            let time_str = captures[2].to_string();
            let clnt = captures[3].to_string();

            // Parse date and time strings into chrono objects
            let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d").expect("Invalid date");
            let time = NaiveTime::parse_from_str(&time_str, "%H%M%S").expect("Invalid time");

            // Create a combined datetime object
            let datetime = NaiveDateTime::new(date, time);
            dbg!(&datetime);

            // Format the datetime object into the desired format
            let formatted_datetime = datetime.format("%Y/%m/%d %H:%M:%S").to_string();

            dbg!(&formatted_datetime);
            (formatted_datetime, clnt)
        }
        None => {
            // Handle the case where the regex does not match.
            log::error!(
                "Could not extract datetime or CLNT from from log path: {}",
                log_path
            );
            (String::new(), String::new())
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
        let re = Regex::new(r"Operation configuration: (\w+(?: \w+)*).*?id: (\d+); Release (\w+)")
            .expect("Unable to parse the operation configuration from the log file");

        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                if let Some(captures) = re.captures(&line) {
                    if let (Some(testtype), Some(id), Some(release)) =
                        (captures.get(1), captures.get(2), captures.get(3))
                    {
                        return Some((
                            testtype.as_str().to_string(),
                            id.as_str().parse().expect("unable to id as string"),
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
