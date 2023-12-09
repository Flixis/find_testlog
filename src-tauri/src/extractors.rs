use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use indexmap::IndexMap;
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

            // Format the datetime object into the desired format
            let formatted_datetime = datetime.format("%Y/%m/%d %H:%M:%S").to_string();

            (formatted_datetime, clnt)
        }
        None => {
            // Handle the case where the regex does not match.
            log::error!(
                "Could not extract datetime or CLNT from from log path: {}",
                log_path
            );
            (
                "Could not determine datetime".to_string(),
                "Could not determine CLNT".to_string(),
            )
        }
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
pub fn extract_info_from_log(log_path_file: &str) -> Option<IndexMap<String, String>> {
    if let Ok(file) = File::open(log_path_file) {
        let reader = io::BufReader::new(file);

        // Initialize an IndexMap(sorted hashmap)
        let mut data = IndexMap::new();

        // Set the maximum number of lines to read
        let max_lines_to_read = 12;

        // Read the file line by line
        let mut line_counter = 0;
        for line in reader.lines() {
            if let Ok(line) = line {
                // Check if the line contains a section header
                if let Some(caps) = regex::Regex::new(r"(\w+):\s*(.+)").unwrap().captures(&line) {
                    let key = caps[1].to_string();
                    let value = caps[2].to_string();
                    if data.contains_key(&key) {
                        let duplicatekey = format!("{}{}", key.clone(), line_counter);
                        data.insert(duplicatekey, value.clone());
                    } else {
                        data.insert(key.clone(), value);
                    }
                }

                line_counter += 1;
                if line_counter >= max_lines_to_read {
                    break;
                }
            }
        }

        //regex for operation configuration splitting
        let re = Regex::new(r"(\w+(?: \w+)*).*?id: (\d+); Release (\w+)")
            .expect("Unable to parse the operation configuration from the log file");

        if let Some(config_text) = data.get("configuration") {
            if let Some(captures) = re.captures(config_text) {
                if let (Some(testtype), Some(id), Some(release)) =
                    (captures.get(1), captures.get(2), captures.get(3))
                {
                    let testtype_str = testtype.as_str().to_string();
                    let id_str = id.as_str().to_string();
                    let release_str = release.as_str().to_string();

                    data.insert("testtype".to_string(), testtype_str);
                    data.insert("id".to_string(), id_str);
                    data.insert("release".to_string(), release_str);
                }
            }
        } else {
            log::error!("Failed to open match regex: {}", log_path_file)
        }

        //return some() because we might return nothing
        Some(data)
    } else {
        log::error!("Failed to open the file: {}", log_path_file);
        None
    }
}
