use std::{println, vec};
use serde::{Serialize, Deserialize};
use clap::Parser;
use colored::*;

/*

find_testlog <SN> <PN>

returns: opens latest logfile


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

    /// PN if not defined it will be pulled from config
    pub pn:Option<String>,

    /// SN if not defined it will be pulled from config
    pub sn:Option<String>,
}


/// `CliAndConfig` implements `Default`
impl Default for CliAndConfig {
    fn default() -> Self { Self { 
        drive: Some("Q:".to_string()), //TODO: set correct drive letter
        location: Some("TestLogs".to_string()), //TODO: set correct default folder
        pn: Some("".to_string()),
        sn: Some("".to_string()),
    } }
}



fn main(){

    let app_name: &str = "find_testlog";
    let _cli_parse = CliAndConfig::parse();

    //Why can't I check if there are no arguments in clap?!?!
    if std::env::args().len() <= 1 {
        let file = confy::get_configuration_file_path(app_name, None).unwrap();
        println!("{} {:#?}", "Configuration file is located at:".red().bold(), file);
        
    }

    load_settings_return_object(&app_name).unwrap();

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
    confy::store(app_name, None, &current_cfg).unwrap();
    find_file_with_params(current_cfg).unwrap();

}


fn load_settings_return_object(app_name:&str) -> Result<CliAndConfig, confy::ConfyError> {
    let cfg: CliAndConfig = confy::load(app_name, None)?;
    return Ok(cfg);
}

///Returns the latest testlog for given pn an sn
fn find_file_with_params(load_settings:CliAndConfig) -> Result<(),()> {
    //We don't have ownership of the struct so we get the reference, then we unwrap so we don't print "Some("Q:")"
    //When there is the option of returning something else other Rust will default to Some(). We can handle this by unwrapping.
    //println!("{:?} {:?} {:?} {:?}", load_settings.drive.as_ref().unwrap(), load_settings.location.as_ref().unwrap(), load_settings.pn.as_ref().unwrap(), load_settings.sn.as_ref().unwrap());
    
    let vec_of_params: Vec<String> = vec![load_settings.drive.unwrap(), load_settings.location.unwrap(), load_settings.pn.unwrap(), load_settings.sn.unwrap(), ];

    // println!("{:?} {:?} {:?} {:?}", load_settings.drive.as_ref().unwrap(), load_settings.location.as_ref().unwrap(), load_settings.pn.as_ref().unwrap(), load_settings.sn.as_ref().unwrap());
    println!("{:?}", vec_of_params);
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