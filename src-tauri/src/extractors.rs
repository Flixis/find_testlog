use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
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
    let mut first_part = read_file_till_bytes(&file, 500);
    let mut second_part = read_file_till_bytes(&file, -500);
    
    first_part = clean_up_string(&first_part);
    second_part = clean_up_string(&second_part);

    
    // dbg!(&first_part);
    println!("{}", &first_part);
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