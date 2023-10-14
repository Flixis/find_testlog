use clap::Parser;
use colored::*;
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
    ///DISABLED Will automatically open the resulting log files, WARNING OPENS ALL OF THEM.
    pub open_log: bool,

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

pub fn execute_cli_args(commandlinearguments: CliCommands) {
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
    }

    if commandlinearguments.folder_location.is_some() {
        search_info.folder_location = commandlinearguments
            .folder_location
            .unwrap_or_default()
            .to_string();
    }

    if commandlinearguments.test_env.is_some() {
        search_info.test_env = commandlinearguments
            .test_env
            .unwrap_or_default()
            .to_string();
    }

    if commandlinearguments.drive_letter.is_some() {
        search_info.drive_letter = commandlinearguments
            .drive_letter
            .unwrap_or_default()
            .to_string();
    }

    //Make sure to save after we've written new data
    if let Err(err) = search_info.save() {
        eprintln!("{} {}", "Failed to save configuration:".red().bold(), err);
    }

    if commandlinearguments.get_config_location {
        let file = confy::get_configuration_file_path("find_testlog", None).unwrap();
        println!(
            "{} {:#?}",
            "Configuration file is located at:".green().bold(),
            file
        );
        exit(0);
    }

    let search_result = crate::functions::search_for_log(&search_info);
    match search_result {
        Ok(paths) => {
            if paths.is_empty() {
                eprintln!(
                    "{} {:?}",
                    "Path could not be matched".red().bold(),
                    search_info
                );
            } else {
                for path in paths {
                    println!("{}", path.replace("\\\\", "\\")); //Remove the double slashes
                }
            }
        }
        _ => eprintln!("{} {:?}", "No matches found: ".red().bold(), search_info),
    }


}
