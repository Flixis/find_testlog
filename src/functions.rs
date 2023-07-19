use std::fs;
use std::io;
use colored::Colorize;
use walkdir::WalkDir;

pub fn itter_find_log(folder_path: String, cli_parse: crate::structs::Cli) -> Result<Vec<String>, io::Error> {
    // Keep track of whether a match is found
    let mut found_match: bool = false;
    let mut log_file_paths: Vec<String> = Vec::new();

    // Iterate over the files in the folder path  
    for entry in WalkDir::new(folder_path) {
        if let Ok(entry) = entry {
            let file_name: String = entry.file_name().to_string_lossy().to_lowercase();
            let sn_lower: String = cli_parse.sn.clone().unwrap_or_default().to_string().to_ascii_lowercase();

            // Check if the file name contains the serial number
            if file_name.contains(&sn_lower) {
                found_match = true;
                // dbg!("{}", entry.path().display());
                if cli_parse.open_log {
                    match open::that(entry.path()) {
                        Ok(()) => println!("{} {}", "Opening Successfully.".green().bold(), entry.path().display()),
                        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, format!("An error occurred when opening {}: {}", entry.path().display(), err))),
                    }
                }
                log_file_paths.push(entry.path().display().to_string());
            }
        }
    }
    if found_match{
        return Ok(log_file_paths)
    }else {
        // If no match is found, return an error
        Err(io::Error::new(io::ErrorKind::NotFound, "No matches found"))
    }
}


pub fn get_most_recent_folder_name(path: &str) -> String {
    // Start by reading all the folders inside the given path
    let min_char_length_folder_name: usize = 7;
    
    let folder_names = fs::read_dir(path)
        .ok()
        .unwrap_or_else(|| {
            eprintln!("Failed to read directory: {}", path);
            fs::read_dir(".").unwrap() // Empty ReadDir iterator
        })
        .filter_map(|entry| { // Build the filter for finding the folders
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            let folder_name = file_name.to_string_lossy().to_string();
            Some(folder_name)
        })
        .filter(|folder_name| { // Now check if the foldername has at least 7 chars or more, if not, it's not relevant.
            folder_name.len() >= min_char_length_folder_name && folder_name[..4].parse::<i32>().is_ok() // Convert the first 4 chars into an INT32, because YYYY format.
        })
        .collect::<Vec<String>>();

    // Now we filter again, but this time we return the highest value folder.
    let most_recent_folder = folder_names.into_iter().max_by_key(|folder| {
        let year = folder[..4].parse::<i32>().unwrap_or(0); // Check the YYYY
        let week = folder[6..].parse::<i32>().unwrap_or(0); // Check the WW
        (year, week)
    });

    most_recent_folder.unwrap_or_else(|| {
        eprintln!("No matching folders found.");
        String::new()
    })
}
