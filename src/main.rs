use std::{println, vec, fs};
use serde::{Serialize, Deserialize};
use clap::Parser;
use colored::*;
use chrono::prelude::*;

/*

find_testlog <SN> <PN>

returns: opens latest logfile

Currently only works with PTF

Q:\TestLogs\6107-2100-6301\2023-W20\PTF\20230515_105021_CLNT4408_group_0_22-39-A2Y-15I.log

 */

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct CliAndConfig {
    /// Sets a custom drive
    #[arg(short, long)]
    pub drive: Option<String>,

    /// Sets a custom directory
    #[arg(short, long)]
    pub location: Option<String>,

    /// Set test-env default is PTF
    #[arg(short, long)]
    pub test_env: Option<String>,

    /// PN if not defined it will be pulled from config
    pub pn:Option<String>,

    /// SN if not defined it will be pulled from config
    pub sn:Option<String>,
}


/// `CliAndConfig` implements `Default`
impl Default for CliAndConfig {
    fn default() -> Self { Self { 
        drive: Some("D:".to_string()), //TODO: set correct drive letter
        location: Some("TestLogs".to_string()), //TODO: set correct default folder
        test_env: Some("PTF".to_string()),
        pn: Some("".to_string()),
        sn: Some("".to_string()),
    } }
}



fn main(){

    let app_name: &str = "find_testlog";
    let _cli_parse = CliAndConfig::parse();
    let date_as_string = Utc::now().to_string();
    println!("{date_as_string}");


    //Why can't I check if there are no arguments in clap?!?!
    if std::env::args().len() <= 1 {
        let file = confy::get_configuration_file_path(app_name, None).unwrap();
        println!("{} {:#?}", "Configuration file is located at:".red().bold(), file);
        
    }

    //load settings into program
    let mut current_cfg: CliAndConfig = confy::load(app_name, None).unwrap();
    
    match &_cli_parse.drive {
        Some(_drive) => {
            // println!("{} {}", "Value for Drive:".purple(), drive);
            current_cfg.drive = _cli_parse.drive;
        }
        None => {
            // println!("{} {:?}", "Using last known Drive:".purple(), current_cfg.drive);
        }
    }
    
    match &_cli_parse.location {
        Some(_location) => {
            // println!("{} {}", "Value for Location:".purple(), location);
            current_cfg.location = _cli_parse.location;
        }
        None => {
            // println!("{} {:?}", "Using last known Location:".purple(), current_cfg.location);
        }
    }

    match &_cli_parse.pn {
        Some(_pn) => {
            // println!("{} {}", "Value for PN:".purple(),pn);
            current_cfg.pn = _cli_parse.pn;
        }
        None => {
            // println!("{} {:?}", "Using last known PN:".purple(),current_cfg.pn);
        }
    }
    
    match &_cli_parse.sn {
        Some(_sn) => {
            // println!("{} {}", "Value for SN:".purple(), sn);
            current_cfg.sn = _cli_parse.sn;
        }
        None => {
            // println!("{} {:?}", "Using last known SN:".purple(), current_cfg.sn);
        }
    }

    //update config file with new values
    confy::store(app_name, None, &current_cfg).unwrap();
    find_file_with_params(&current_cfg).unwrap();

}


///Returns the latest testlog for given pn an sn
fn find_file_with_params(load_settings:&CliAndConfig) -> Result<(),()> {
    //We don't have ownership of the struct so we get the reference, then we unwrap so we don't print "Some("Q:")"
    //When there is the option of returning something else , Rust will default to Some(). We can handle this by unwrapping.

    let path_to_find_latest_folder: Vec<&str> = vec![
        load_settings.drive.as_ref().unwrap(),
        load_settings.location.as_ref().unwrap(),
        load_settings.pn.as_ref().unwrap()
    ];

    let paths = fs::read_dir(path_to_find_latest_folder.join("\\")).unwrap();
    let mut latest_week_year_folder = String::new();

    for path in paths {
        if let Some(entry) = path.ok() {
            latest_week_year_folder = entry.path().to_string_lossy().to_string();
        }
    }
    
    dbg!("Latest Week Year Folder: {}", latest_week_year_folder);
    
    


    let full_path_to_latest_week: Vec<&str> = vec![
        load_settings.drive.as_ref().unwrap(),
        load_settings.location.as_ref().unwrap(),
        load_settings.pn.as_ref().unwrap(),
        load_settings.test_env.as_ref().unwrap(),
        "weekyear",
        load_settings.sn.as_ref().unwrap(),
    ];
    let _concatenated_string = full_path_to_latest_week.join("\\");
    // println!("{}", concatenated_string);
    Ok(())
}

/* 
/// Parses Cli commands and automatically updates config file
// Note: this is over-engineered
// however I wanted to prove to myself that I have a decent understanding of rust-lang at this point.
fn match_cli_parser<T>(object: &T , mut current_cfg:CliAndConfig , _cli_parse:CliAndConfig){
    match &_cli_parse.object {
        Some(object) => {
            println!("{} {} {}", "Value for".purple(), object, object);
            current_cfg.object = _cli_parse.object;
        }
        None => {
            println!("{} {:?}", "Using last known:".purple(), current_cfg.drive);
        }
    }
    todo!()
}
*/