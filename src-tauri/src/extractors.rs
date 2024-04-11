use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

/*

Pattern matches on datetime and clnt, which is required to build a valid date-time string.

*/
/// Extracts date and time from filepath and parses it as chrono:naivedate
pub fn extract_datetime_clnt_from_logpath(log_path: &str) -> (String, String) {
    
    let log_path =  log_path.split('/').last().ok_or_else(|| log::error!("Failed to extract log filename from path")).unwrap();
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
    _text_keep_amount: usize,
) -> Result<String, io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            log::error!("Failed to open file: {}", err);
            return Err(err.into());
        }
    };

    //500 bytes is just a random number I picked, it could be moved higher if the need is there
    let mut first_part = read_file_till_bytes(&file, 500);
    let mut second_part = read_file_till_bytes(&file, -500);
    first_part = clean_up_string(&first_part);
    second_part = clean_up_string(&second_part);



    let cleaned_operation_headers = create_header_hashmap_from_headers_string(&first_part);
    let cleaned_operation_status = create_header_hashmap_from_headers_string(&second_part);

    
    // dbg!(&first_part);
    dbg!(&cleaned_operation_headers);
    // dbg!(&second_part);
    println!("{}", &second_part);
    
    Ok(String::new())

}

fn read_file_till_bytes(mut file: &File, bytes_to_read: i64) -> String{
    
    if bytes_to_read < 0 {
        file.seek(SeekFrom::End(bytes_to_read)).unwrap();
    }
    let mut buffer = vec![0; bytes_to_read.abs() as usize];
    let n = file.read(&mut buffer).unwrap();
    
    // Convert the buffer to a String, handling potential errors.
    let text = String::from_utf8(buffer[..n].to_vec()).unwrap();
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
                dbg!(&parts);
                if !parts.is_empty() {
                    hashmap.insert("operation_configuration".to_string(), parts[0].to_string());

                    // Further processing for id and Release parts
                    for part in &parts[1..] {
                        if part.starts_with("(id:") {
                            let id = parts[2].to_string();
                            hashmap.insert("id".to_string(), id);
                        } else if part.starts_with("Release") {
                            let release = parts[4..6].concat().to_string();
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