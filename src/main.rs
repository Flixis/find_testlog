use std::env;
use walkdir::WalkDir;

//cargo run -- D: TestLogs 6107-2100-6301 2023-W51 PTF 22-39-A2Y-15I
fn main() {
    // Read command line arguments
    let drive_letter = env::args().nth(1).unwrap_or(String::from("D:"));
    let folder_location = env::args().nth(2).unwrap_or(String::from("TestLogs"));
    let pn = env::args().nth(3).unwrap_or(String::from("6107-2100-6301"));
    let week_year = env::args().nth(4).expect("Missing week-year argument");
    let test_env = env::args().nth(5).unwrap_or(String::from("PTF"));
    let sn = env::args().nth(6).expect("Missing serial number argument");

    // Build the folder path
    let folder_path = format!(
        "{}\\{}\\{}\\{}\\{}",
        drive_letter, folder_location, pn, week_year, test_env
    );

    // Iterate over the files in the folder path
    for entry in WalkDir::new(folder_path) {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            let sn_lower = sn.to_lowercase();

            // Check if the file name contains the serial number
            if file_name.contains(&sn_lower) {
                println!("{}", entry.path().display());
            }
        }
    }
}
