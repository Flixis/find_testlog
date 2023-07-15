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




Now you can run the utility (from the project root):

```bash
./target/release/find-testlog -d "D:" -f "TestLogs" -p "6107-2100-6301" -y "2023-W51" -t "PTF" -s "22-39-A2Y-15I"
```



### Contribution

Contributions are welcome! Please feel free to submit a Pull Request.
License

This project is licensed under the MIT License - see the LICENSE file for details.