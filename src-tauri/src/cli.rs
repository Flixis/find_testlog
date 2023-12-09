use clap::Parser;
use colored::*;
use indexmap::indexmap;
use std::io;
use std::process::exit;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliCommands {
    #[clap(short, long)]
    ///Product Number, Example: 9999-1111-2222.
    pub productnumber: Option<String>,

    #[clap(short, long)]
    ///Serial Number, Example: 99-11-AAA-BBB
    pub serialnumber: Option<String>,

    #[clap(short, long)]
    ///Year Week, Example: 2023-W51, Defaults: Searches all year-week folders.
    pub year_week: Option<String>,

    #[clap(short, long)]
    ///Test environment, Default: PTF
    pub test_env: Option<String>,

    #[clap(short, long)]
    ///Drive letter, Default Q:
    pub drive_letter: Option<String>,

    #[clap(short, long)]
    ///Folder location, Default: TestLogs.
    pub folder_location: Option<String>,

    #[clap(short, long)]
    ///If passed, Returns config location
    pub get_config_location: bool,
}

pub fn parse_cli_args(commandlinearguments: CliCommands) -> crate::structs::AppConfig {
    let mut search_info = crate::structs::AppConfig::default_values();

    //Make sure that the arguments have data else just ignore.
    if commandlinearguments.productnumber.is_some() {
        search_info.productnumber = commandlinearguments
            .productnumber
            .unwrap_or_default()
            .to_string();
    }
    if commandlinearguments.serialnumber.is_some() {
        search_info.serialnumber = commandlinearguments
            .serialnumber
            .unwrap_or_default()
            .to_string();
    }

    if commandlinearguments.year_week.is_some() {
        search_info.dateyyyyww = commandlinearguments
            .year_week
            .unwrap_or_default()
            .to_string();
    } else {
        search_info.dateyyyyww = "".to_string();
    }

    if commandlinearguments.folder_location.is_some() {
        search_info.folder_location = commandlinearguments
            .folder_location
            .unwrap_or_default()
            .to_string();
    }

    if commandlinearguments.test_env.is_some() {
        search_info.test_suite = commandlinearguments
            .test_env
            .unwrap_or_default()
            .to_string();
    } else {
        search_info.test_suite = "".to_string();
    }

    if commandlinearguments.drive_letter.is_some() {
        search_info.drive_letter = commandlinearguments
            .drive_letter
            .unwrap_or_default()
            .to_string();
    }

    //Make sure to save after we've written new data
    if let Err(err) = search_info.save() {
        log::error!("{} {}", "Failed to save configuration:".red().bold(), err);
    }

    if commandlinearguments.get_config_location {
        let file = confy::get_configuration_file_path("find_testlog", None)
            .expect("Failed to get configuration");
        println!(
            "{} {:#?}",
            "Configuration file is located at:".green().bold(),
            file
        );
        exit(0);
    }

    search_info
}

pub fn execute_search_results_from_cli(search_info: crate::structs::AppConfig) {
    // using indexmap crate because there is no way to order std::hashmaps.
    let mut mapped_search_results = indexmap! {};
    // Search for log files based on the search criteria.
    let search_result = crate::search::search_for_log(&search_info);
    let mut key_counter: i16 = 0;

    match search_result {
        Ok(paths) => {
            // If no log files were found, print an error message.
            if paths.is_empty() {
                log::error!(
                    "{} {:?}",
                    "Path could not be matched".red().bold(),
                    search_info
                );
            } else {
                // Iterate over the found log files and add them to a hashmap,
                // where the key is a counter and the value is the file path.
                for path in paths {
                    key_counter += 1;
                    mapped_search_results.insert(key_counter, path);
                }
            }
        }

        _ => log::error!("{} {:?}", "No matches found: ".red().bold(), search_info),
    }

    // If one or more log file was found, prompt the user to select
    // which one they want to open.
    if key_counter >= 1 {
        for (key, value) in &mapped_search_results {
            println!("#{key} {value}");
        }

        println!("Please select #.. to open");

        // Read the user's input and try to parse it as an integer.
        let mut input_string = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read from stdin");
        let input_to_int = input_string.trim().parse::<i16>();

        // If the user's input was successfully parsed, get the file path
        // at the corresponding index in the map.
        let open_file: Option<&String> = match input_to_int {
            Ok(i) => mapped_search_results.get(&i),
            Err(..) => {
                log::error!("{}", "Couldn't parse".red().bold());
                None
            }
        };

        // If a file path was found, try to open the file.
        if let Some(path) = open_file {
            open::that(path).expect("Failed to open the file");
        } else {
            log::error!("{}", "Invalid file path".red().bold());
        }
    } else {
        // if key_counter is 0 then just exit.
        exit(-1);
    }
}
