use serde::{Serialize, Deserialize};
use clap::Parser;

#[derive(Parser, Debug, Serialize, Deserialize)]
struct MyConfig {
    /// Optional name to operate on
    #[arg(short, long)]
    drive: Option<String>,

    /// Sets a custom config file
    #[arg(short, long)]
    location: Option<String>,

    /// Required PN if not defined it will be pulled from config
    pn:Option<String>,

    /// Required SN if not defined it will be pulled from config
    sn:Option<String>,
}


/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { 
        drive: Some("k".to_string()), 
        location: Some("k".to_string()),
        pn: Some("".to_string()),
        sn: Some("".to_string()),
    } }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: MyConfig = confy::load("my-app-name", None)?;
    dbg!(cfg);
    Ok(())
}