use std::{println};
use serde::{Serialize, Deserialize};
use clap::Parser;
use colored::*;

/*

find_testlog <SN> <PN>

returns: list of 5 latest testresults

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

    if let Some(pn) = _cli_parse.pn.as_deref() {
        println!("Value for PN: {pn}");
    }

    if let Some(sn) = _cli_parse.sn.as_deref() {
        println!("Value for SN: {sn}");
    }
}