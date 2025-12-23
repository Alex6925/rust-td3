use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use regex::Regex;
use serde::Serialize;
use prettytable::{Table, Row, Cell};



#[derive(Parser, Debug)]
#[command(name = "loglyzer")]
#[command(version = "1.0")]
#[command(about = "Analyze log files and extract patterns", long_about = None)]
struct Cli {
    /// Path to the log file to analyze
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output format: text, json, csv
    #[arg(short, long, value_enum, default_value = "text")]
    format: OutputFormat,

    /// Show only ERROR-level logs
    #[arg(short, long)]
    errors_only: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show top N most frequent errors
    #[arg(long, default_value = "5")]
    top: usize,

    /// Filter logs containing specific text (case-insensitive)
    #[arg(long)]
    search: Option<String>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

/* =========================
   Log structures — Part 2
   ========================= */

#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: String,
    level: LogLevel,
    message: String,
}

#[derive(Debug, Clone, PartialEq)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

impl LogLevel {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "INFO" => Some(LogLevel::Info),
            "WARNING" | "WARN" => Some(LogLevel::Warning),
            "ERROR" => Some(LogLevel::Error),
            "DEBUG" => Some(LogLevel::Debug),
            _ => None,
        }
    }
}



fn read_log_file(path: &std::path::Path) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}



fn parse_log_line(line: &str) -> Option<LogEntry> {
    let re = Regex::new(
        r"^(\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2})\s+\[(\w+)\]\s+(.+)$"
    ).ok()?;

    let caps = re.captures(line)?;

    Some(LogEntry {
        timestamp: caps.get(1)?.as_str().to_string(),
        level: LogLevel::from_str(caps.get(2)?.as_str())?,
        message: caps.get(3)?.as_str().to_string(),
    })
}



#[derive(Debug, Serialize)]
struct ErrorFrequency {
    message: String,
    count: usize,
}

#[derive(Debug, Serialize)]
struct LogStats {
    total_entries: usize,
    by_level: HashMap<String, usize>,
    top_errors: Vec<ErrorFrequency>,
}

fn analyze_logs(entries: &[LogEntry], top_n: usize) -> LogStats {
    let mut by_level: HashMap<String, usize> = HashMap::new();
    let mut error_messages: HashMap<String, usize> = HashMap::new();

    for entry in entries {
        let level_name = format!("{:?}", entry.level);
        *by_level.entry(level_name).or_insert(0) += 1;

        if entry.level == LogLevel::Error {
            *error_messages.entry(entry.message.clone()).or_insert(0) += 1;
        }
    }

    let mut top_errors: Vec<ErrorFrequency> = error_messages
        .into_iter()
        .map(|(message, count)| ErrorFrequency { message, count })
        .collect();

    top_errors.sort_by(|a, b| b.count.cmp(&a.count));
    top_errors.truncate(top_n);

    LogStats {
        total_entries: entries.len(),
        by_level,
        top_errors,
    }
}



fn output_text(stats: &LogStats) {
    println!("\nLog Analysis Results");
    println!("====================");
    println!("Total entries: {}\n", stats.total_entries);

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Level"),
        Cell::new("Count"),
    ]));

    for (level, count) in &stats.by_level {
        table.add_row(Row::new(vec![
            Cell::new(level),
            Cell::new(&count.to_string()),
        ]));
    }

    table.printstd();

    if !stats.top_errors.is_empty() {
        println!("\nTop errors:");
        let mut err_table = Table::new();
        err_table.add_row(Row::new(vec![
            Cell::new("Message"),
            Cell::new("Occurrences"),
        ]));

        for err in &stats.top_errors {
            err_table.add_row(Row::new(vec![
                Cell::new(&err.message),
                Cell::new(&err.count.to_string()),
            ]));
        }

        err_table.printstd();
    }
}

fn output_json(stats: &LogStats) {
    let json = serde_json::to_string_pretty(stats).unwrap();
    println!("{}", json);
}

fn output_csv(stats: &LogStats) {
    println!("level,count");
    for (level, count) in &stats.by_level {
        println!("{},{}", level, count);
    }
}



fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Analysing file: {:?}", cli.input);
        println!("Format: {:?}", cli.format);
        println!("Top errors: {}", cli.top);
        println!("Search filter: {:?}", cli.search);
    }

    let lines = match read_log_file(&cli.input) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("❌ Failed to read file: {}", e);
            std::process::exit(1);
        }
    };

    let parsed: Vec<LogEntry> = lines
        .iter()
        .filter_map(|line| parse_log_line(line))
        .collect();

    let filtered: Vec<LogEntry> = parsed
        .into_iter()
        .filter(|e| !cli.errors_only || e.level == LogLevel::Error)
        .filter(|e| {
            if let Some(ref needle) = cli.search {
                let needle = needle.to_lowercase();
                e.message.to_lowercase().contains(&needle)
                    || e.timestamp.to_lowercase().contains(&needle)
                    || format!("{:?}", e.level).to_lowercase().contains(&needle)
            } else {
                true
            }
        })
        .collect();

    let stats = analyze_logs(&filtered, cli.top);

    match cli.format {
        OutputFormat::Text => output_text(&stats),
        OutputFormat::Json => output_json(&stats),
        OutputFormat::Csv => output_csv(&stats),
    }
}