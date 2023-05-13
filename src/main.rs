use std::fs;
use clap::Parser;


/*

find_testlog -pn 6001-xxxx-xxxx -sn xxx-xxx-xxx

 */


 #[derive(Parser)]
 #[command(author, version, about, long_about = None)]
 struct Cli {
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




fn main() {

    let cli_parse = Cli::parse();


    let parsed_json = parse_json("find_testlog_config.json".to_string());

    parse_settings(&parsed_json, cli_parse);

    println!("{}", &parsed_json);
}




fn parse_json(file_path:String) -> serde_json::value::Value {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let json_data = fs::read_to_string(&file_path).expect("couldn't read");

    // Parse the string of data into serde_json::Value.
    let json_serialized: Value = serde_json::from_str(&json_data).unwrap();

    return json_serialized;
}


fn parse_settings(json_serialized:&serde_json::value::Value, cli_arguments:Cli){

    let Some(drive_cli) = cli_arguments.drive;
    
   
    match (cli_arguments.drive, json_serialized.get("drive")) {
        (None, _) =>{
            println!("Couldn't find drive in config or as an argument.");
        }
        (_, Some(drive)) => {
            println!("Using drive: {}", drive);
        }
        (_, None) => {
            println!("Using drive: {}", );
        }
    }

    
    
}



//Helps me figure out what types are floating around :)
#[warn(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}