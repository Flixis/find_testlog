use std::println;

use serde::{Serialize, Deserialize};
use clap::Parser;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
struct CliAndConfig {
    /// Sets a custom drive
    #[arg(short, long)]
    drive: Option<String>,

    /// Sets a custom directory
    #[arg(short, long)]
    location: Option<String>,

    /// PN if not defined it will be pulled from config
    pn:Option<String>,

    /// SN if not defined it will be pulled from config
    sn:Option<String>,
}


/// `CliAndConfig` implements `Default`
impl ::std::default::Default for CliAndConfig {
    fn default() -> Self { Self { 
        drive: Some("k".to_string()), 
        location: Some("k".to_string()),
        pn: Some("".to_string()),
        sn: Some("".to_string()),
    } }
}


fn handle_settings(app_name:&str) -> Result<(), confy::ConfyError> {
    let cfg: CliAndConfig = confy::load(app_name, None)?;

    // dbg!(cfg);
    Ok(())
}

fn main(){

    let app_name: &str = "find_testlog";
    let cli_parse = CliAndConfig::parse();

    //Why can't I check if there are no arguments in clap?!?!
    if std::env::args().len() == 1 {
        let file = confy::get_configuration_file_path("confy_simple_app", None).unwrap();
        println!("The configuration file path is: {:#?}", file);
    }
    
    handle_settings(&app_name).unwrap();
}