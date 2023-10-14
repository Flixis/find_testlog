use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliCommands {
    #[clap(short, long)]
    ///Product Number, Example: 6107-2100-6301.
    pub productnumber: Option<String>,

    #[clap(short, long)]
    ///Serial Number, Example: 22-39-A2Y-15I
    pub serialnumber: Option<String>,

    #[clap(short, long)]
    ///Year Week, Example: 2023-W51, Defaults: Searches all year-week folders.
    pub year_week: Option<String>,

    #[clap(short, long)]
    ///Test environment, Default: PTF
    pub test_env: Option<String>,

    #[clap(short, long)]
    ///Will automatically open the resulting log files, WARNING OPENS ALL OF THEM.
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
