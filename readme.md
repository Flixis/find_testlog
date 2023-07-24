# Find TestLog

This is a command-line utility developed in Rust, designed to assist in finding and opening log files in a complex directory structure. This tool can be extremely useful if you have a directory structure based on product numbers, year-weeks, and test environments. The application uses a configuration file to save and reuse settings, enhancing its usability.

## Features
- Search for log files based on parameters like drive letter, folder location, product number, year-week, test environment, and serial number.
- Automatically open the found log files (optional).
- Store the last used parameters in a configuration file for future use. These stored parameters will be used for subsequent searches unless new ones are provided.
- Retrieve the location of the configuration file.
- Command-line arguments take precedence over the configuration file.
- Load and save application configuration from a file.
- Easy interaction through a command-line interface.

## Usage

The application accepts several command-line arguments:

    -p, --pn <pn>: Product Number (Example: 9999-1234-5678).
    -s, --sn <sn>: Serial Number (Example: xx-xx-yyy-000).
    -y, --year_week <year_week>: Year Week (Example: 2023-W51). Defaults to searching all year-week folders.
    -t, --test_env <test_env>: Test environment. Default is PTF.
    -o, --open_log: If passed, will automatically open the resulting log files. WARNING: OPENS ALL OF THEM.
    -d, --drive_letter <drive_letter>: Drive letter. Default is Q:.
    -f, --folder_location <folder_location>: Folder location. Default is TestLogs.
    -g, --get_config_location: If passed, Returns config location.

If the tool has been run at least once, and no arguments are provided for the subsequent run, it will utilize the parameters stored in the configuration file from the last run:

```bash
./target/release/find-testlog
```

## Getting Started

To use this tool, you need to have Rust installed on your system. If you don't, visit rust-lang.org/tools/install to install Rust.

Once you have Rust installed, you can clone this repository and build the project:
```bash
git clone https://github.com/yourusername/find-testlog.git
cd find-testlog
cargo build --release
```

[Or You can download the latest version from the releases page.](https://github.com/Flixis/find_testlog/releases)



### Quickstart
Now you can run the utility (from the project root):
```bash
./target/release/find-testlog -d "D:" -f "TestLogs" -p "REPLACE_PRODUCT_NUMBER" -y "2023-W51" -t "PTF" -s "REPLACE_SERIAL_NUMBER"

output:
Searching inside: 2023-W51
Matched log file paths:
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W51\PTF\20230515_105021_CLNTXXXX_group_0_REPLACE_SERIAL_NUMBER.log
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W51\PTF\20240515_105021_CLNTXXXX_group_0_REPLACE_SERIAL_NUMBER.log
```

Only passing the ``SN`` will make the CLI tool search all folders.

```bash
./target/release/find-testlog -s "REPLACE_SERIAL_NUMBER"

output:
Year-week not specified, searching all folders.
Matched log file paths:
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W20\PTF\20230515_105021_CLNTXXXX_group_0_REPLACE_SERIAL_NUMBER - Copy.log
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W20\PTF\20230515_105021_CLNTXXXX_group_0_REPLACE_SERIAL_NUMBER.log
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W51\PTF\20230515_105021_CLNTXXXX_group_0_REPLACE_SERIAL_NUMBER.log
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W51\PTF\20240515_105021_CLNTXXXX_group_0_REPLACE_SERIAL_NUMBER.log
```


### Configuration and Persistent Parameters

Once find-testlog has been run once, it will save the provided parameters into a configuration file. The next time the application is run, if no arguments are provided, find-testlog will pull parameters from this configuration file, allowing for quicker and more convenient usage.

Here's how it works:

1. Run the find-testlog application with your desired parameters. For example:
```bash
./target/release/find-testlog -d "D:" -f "TestLogs" -p "REPLACE_PRODUCT_NUMBER" -y "2023-W51" -t "PTF" -s "REPLACE_SERIAL_NUMBER"

output:
Searching inside: 2023-W51
Matched log file paths:
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W51\PTF\20230515_105021_CLNT4408_group_0_REPLACE_SERIAL_NUMBER.log
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W51\PTF\20240515_105021_CLNT4408_group_0_REPLACE_SERIAL_NUMBER.log
```
The application will now save these parameters into the configuration file.

2. The next time you need to search for log files with the same parameters, you can simply run:

```bash
Note: When running with no params the program will search all folders.

.\find_testlog.exe
Year-week not specified, searching all folders.
Matched log file paths:
D:\TestLogs\REPLACE_PRODUCT_NUMBER\
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2021-W19
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2022-W43
......
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W20
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W20\PTF
D:\TestLogs\REPLACE_PRODUCT_NUMBER\2023-W51\PTF\20240515_105021_CLNT4408_group_0_REPLACE_SERIAL_NUMBER.log
```    
The application will automatically pull the parameters from the configuration file and use them for the search.

If you wish to override some or all parameters stored in the configuration file, simply provide the new values as command-line arguments. For example:

```bash
./target/release/find-testlog -p "REPLACE_PRODUCT_NUMBER"
```
In this case, find-testlog will use the new product number but will pull all other parameters from the configuration file.

To see where your configuration file is located, run:

```bash
./target/release/find-testlog --get-config-location

Configuration file is located at: "C:\\Users\\User\\AppData\\Roaming\\find_testlog\\config\\default-config.toml"
```

The configuration file can be edited manually if needed. However, it is recommended to change parameters using the command-line arguments, as this ensures that the configuration file remains in a valid state.

Remember, the use of the configuration file can streamline your workflow, particularly when frequently searching for log files with the same parameters.

## Dependencies

- [clap: For building command-line interfaces.](https://docs.rs/crate/clap/4.3.17)
- [confy: For handling application configuration.](https://docs.rs/crate/confy/0.5.1)
- [serde: For serializing and deserializing Rust data structures.](https://docs.rs/crate/serde/1.0.163)
- [colored: For coloring terminal text.](https://docs.rs/crate/colored/2.0.4)
- [walkdir: For walking directory trees.](https://docs.rs/crate/walkdir/2.3.3)
- [open: path or URL using the program configured on the system. ](https://docs.rs/crate/open/5.0.0)

### Contribution

Contributions are welcome! Please feel free to submit a Pull Request.
License

This project is licensed under the GNU General Public License v3.0 License - see the LICENSE file for details.
