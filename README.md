# Rust TD3 – Log Analyzer CLI

This project is a command-line log analyzer developed in Rust as part of **TD3**.  
It reads structured log files, extracts relevant information, and provides  
statistical analysis through a professional CLI interface.

## Features

- Command-line interface built with `clap`
- Efficient file reading using `BufReader`
- Log parsing with regular expressions
- Filtering options:
  - `--errors-only` to display only error-level logs
  - `--search <text>` to filter logs containing a specific keyword (case-insensitive)
- Log analysis:
  - Total number of entries
  - Count of entries by log level (INFO, WARNING, ERROR, DEBUG)
  - Top N most frequent error messages
- Multiple output formats:
  - Text (formatted tables)
  - JSON
  - CSV

## Supported Log Format

Each log line must follow the format:

YYYY-MM-DD HH:MM:SS [LEVEL] Message

Example:

2024-01-15 10:31:15 [ERROR] Database query failed: syntax error

## Usage

### Basic analysis

cargo run – sample.log

### Verbose mode

cargo run –– verbose sample.log

### Show only errors

cargo run –– errors-only sample.log

### Search for a keyword

cargo run –– search database sample.log

### JSON output

cargo run –– format json sample.log

### CSV output

cargo run –– format csv sample.log

### Combined options

cargo run –– errors-only –top 10 –format json sample.log

## Project Structure

rust-td3/
├── src/
│   └── main.rs
├── sample.log
├── Cargo.toml
├── .gitignore
└── README.md

## Technologies Used

- Rust
- clap
- regex
- serde / serde_json
- prettytable-rs

## Author

Alexandre Fau  
ESILV