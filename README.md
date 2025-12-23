# Rust TD3 – Log Analyzer CLI

This project is a command-line log analyzer developed in Rust as part of **TD3**.
It parses structured log files, extracts useful information, and provides
statistics in multiple output formats.

## Features

- Command-line interface built with `clap`
- Efficient file reading with `BufReader`
- Log parsing using regular expressions
- Filtering options:
  - `--errors-only`
  - `--search <text>` (case-insensitive)
- Statistical analysis:
  - Count of log entries by level
  - Top N most frequent error messages
- Multiple output formats:
  - Text (table)
  - JSON
  - CSV

## Supported Log Format

Each log line must follow this format: