use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use indexmap::IndexMap;
use regex::Regex;
use std::collections::VecDeque;
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

            log::debug!("extract_datetime_clnt_from_logpath: {}{}",formatted_datetime, clnt);
            (formatted_datetime, clnt)
        }
        None => {
            // Handle the case where the regex does not match.
            log::error!(
                "Could not extract datetime or CLNT from from log path: {}",
                log_path
            );
            (
                //Returns a new string, the 'extract_info_from_log' function allows us to recoup this data.
                String::new(),
                String::new(),
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

pub fn extract_info_from_log(
    filename: &str,
    text_keep_amount: usize,
) -> Result<Option<IndexMap<String, String>>, io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            log::error!("Failed to open file: {}", err);
            return Err(err.into());
        }
    };
    let reader = io::BufReader::new(file);

    let mut first_lines = Vec::new();
    let mut last_lines = VecDeque::new();

    for (index, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => {
                log::error!("Error reading line {}: {}", index, err);
                continue; // Skip this line and continue with the next one
            }
        };

        if index < text_keep_amount {
            first_lines.push(line.clone());
        }
        last_lines.push_back(line.clone());

        // Keep only the last x lines in the last_lines queue.
        while last_lines.len() > text_keep_amount {
            last_lines.pop_front();
        }
    }

    let mut data: IndexMap<String, String> = IndexMap::new();
    let re_first = Regex::new(r"(\w+):\s*(.+)").unwrap();
    let re_last = Regex::new(r"\b(PASS(?:ED)?|FAIL(?:ED)?)\b").unwrap();

    for line in first_lines {
        if let Some(caps) = re_first.captures(&line) {
            let key = caps[1].to_string();
            let value = caps[2].to_string();
            if data.contains_key(&key) {
                let duplicatekey = format!("{}{}", key.clone(), "_duplicate");
                data.insert(duplicatekey, value.clone());
            } else {
                data.insert(key.clone(), value);
            }
        }
    }

    for line in last_lines {
        if let Some(caps) = re_last.captures(&line) {
            let key = "PASS_FAIL_STATUS".to_string();
            let value = caps[1].to_string();
            if data.contains_key(&key) {
                let duplicatekey = format!("{}{}", key.clone(), "_duplicate");
                data.insert(duplicatekey, value.clone());
            } else {
                data.insert(key.clone(), value);
            }
        }
    }

    // Regex for operation configuration splitting
    let config_re = Regex::new(r"(\w+(?: \w+)*).*?id: (\d+); Release (\w+)")
        .expect("Unable to parse the operation configuration from the log file");

    if let Some(config_text) = data.get("configuration") {
        if let Some(captures) = config_re.captures(config_text) {
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
        log::error!("Failed to find 'configuration' field in the log.");
        return Ok(Some(data));
    }
    log::info!("extract_info_from_log: {:?}",data);
    Ok(Some(data))
}
