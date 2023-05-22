use std::{println};
use serde::{Serialize, Deserialize};
use clap::Parser;
use colored::*;

/*

find_testlog <SN> <PN>

returns: opens latest logfile

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

    /// PN if not defined it will be pulled from config
    pub pn:Option<String>,

    /// SN if not defined it will be pulled from config
    pub sn:Option<String>,
}


/// `CliAndConfig` implements `Default`
impl Default for CliAndConfig {
    fn default() -> Self { Self { 
        drive: Some("d:".to_string()), //TODO: set correct drive letter
        location: Some("testlogs".to_string()), //TODO: set correct default folder
        pn: Some("".to_string()),
        sn: Some("".to_string()),
    } }
}


fn handle_settings(app_name:&str) -> Result<(), confy::ConfyError> {
    let _cfg: CliAndConfig = confy::load(app_name, None)?;
    Ok(())
}


fn main(){

    let app_name: &str = "find_testlog";
    let _cli_parse = CliAndConfig::parse();

    //Why can't I check if there are no arguments in clap?!?!
    if std::env::args().len() <= 1 {
        let file = confy::get_configuration_file_path("confy_simple_app", None).unwrap();
        println!("{} {:#?}", "Configuration file is located at:".red().bold(), file);
        
    }

    handle_settings(&app_name).unwrap();

    //load settings into program
    let mut current_cfg: CliAndConfig = confy::load(app_name, None).unwrap();
    
    
    match &_cli_parse.drive {
        Some(drive) => {
            println!("{} {}", "Value for Drive:".purple(), drive);
            current_cfg.drive = _cli_parse.drive;
        }
        None => {
            println!("{} {:?}", "Using last known Drive:".purple(), current_cfg.drive);
        }
    }
    
    match &_cli_parse.location {
        Some(location) => {
            println!("{} {}", "Value for Location:".purple(), location);
            current_cfg.location = _cli_parse.location;
        }
        None => {
            println!("{} {:?}", "Using last known Location:".purple(), current_cfg.location);
        }
    }

    match &_cli_parse.pn {
        Some(pn) => {
            println!("{} {}", "Value for PN:".purple(),pn);
            current_cfg.pn = _cli_parse.pn;
        }
        None => {
            println!("{} {:?}", "Using last known PN:".purple(),current_cfg.pn);
        }
    }
    
    match &_cli_parse.sn {
        Some(sn) => {
            println!("{} {}", "Value for SN:".purple(), sn);
            current_cfg.sn = _cli_parse.sn;
        }
        None => {
            println!("{} {:?}", "Using last known SN:".purple(), current_cfg.sn);
        }
    }


    

    //update config file with new values
    confy::store(app_name, None, current_cfg).unwrap();



}