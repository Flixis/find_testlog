# Find TestLog

This is a command-line utility developed in Rust, designed to assist in finding and opening log files in a complex directory structure. This tool can be extremely useful if you have a directory structure based on product numbers, year-weeks, and test environments. The application uses a configuration file to save and reuse settings, enhancing its usability.

## Features
- Search for log files based on parameters like drive letter, folder location, product number,  year-week, test environment, and serial number.
- Automatically open log files found (optional).
- Remembers the last used parameters, stored in a configuration file for future use. These stored parameters will be used for subsequent searches unless new ones are provided.
- Retrieve the location of the configuration file.
- Command-line arguments take precedence over the configuration file.

## Usage

The application accepts several command-line arguments:

    -d, --drive-letter - Drive letter (Example: D:)
    -f, --folder-location - Folder location (Example: TestLogs)
    -p, --pn - Product Number (Example: 6107-2100-6301)
    -y, --year-week - Year Week (Example: 2023-W51). Defaults to the latest year-week if not provided.
    -t, --test-env - Test environment (Example: PTF)
    -s, --sn - Serial Number (Example: 22-39-A2Y-15I)
    -g, --get-config-location - Returns the location of the configuration file.
    -o, --open-log - Automatically open the resulting log files. WARNING: This will open all of them.

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


Now you can run the utility (from the project root):

```bash
./target/release/find-testlog -d "D:" -f "TestLogs" -p "6107-2100-6301" -y "2023-W51" -t "PTF" -s "22-39-A2Y-15I"
```


### Configuration and Persistent Parameters

Once find-testlog has been run once, it will save the provided parameters into a configuration file. The next time the application is run, if no arguments are provided, find-testlog will pull parameters from this configuration file, allowing for quicker and more convenient usage.

Here's how it works:

1. Run the find-testlog application with your desired parameters. For example:
```bash
./target/release/find-testlog -d "D:" -f "TestLogs" -p "6107-2100-6301" -y "2023-W51" -t "PTF" -s "22-39-A2Y-15I"

output:
D:\TestLogs\6107-2100-6301\2023-W51\PTF\20230515_105021_CLNT4408_group_0_22-39-A2Y-15I.log
D:\TestLogs\6107-2100-6301\2023-W51\PTF\20240515_105021_CLNT4408_group_0_22-39-A2Y-15I.log
```
The application will now save these parameters into the configuration file.

2. The next time you need to search for log files with the same parameters, you can simply run:

```bash
./target/release/find-testlog

output:
D:\TestLogs\6107-2100-6301\2023-W51\PTF\20230515_105021_CLNT4408_group_0_22-39-A2Y-15I.log
D:\TestLogs\6107-2100-6301\2023-W51\PTF\20240515_105021_CLNT4408_group_0_22-39-A2Y-15I.log
```    
The application will automatically pull the parameters from the configuration file and use them for the search.

If you wish to override some or all parameters stored in the configuration file, simply provide the new values as command-line arguments. For example:

```bash
./target/release/find-testlog -p "6107-2100-6302"
```
In this case, find-testlog will use the new product number but will pull all other parameters from the configuration file.

To see where your configuration file is located, run:

```bash
./target/release/find-testlog --get-config-location
```

The configuration file can be edited manually if needed. However, it is recommended to change parameters using the command-line arguments, as this ensures that the configuration file remains in a valid state.

Remember, the use of the configuration file can streamline your workflow, particularly when frequently searching for log files with the same parameters.

### Contribution

Contributions are welcome! Please feel free to submit a Pull Request.
License

This project is licensed under the GNU General Public License v3.0 License - see the LICENSE file for details.
