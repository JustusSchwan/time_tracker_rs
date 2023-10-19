use clap::{Parser, Subcommand};
use std::str::FromStr;
use chrono::{NaiveTime, Timelike};
use dirs;
use std::path::Path;
use csv;
use serde::{Serialize, Deserialize};
use std::vec::Vec;
use std::fs::File;

/// Keep track of what have worked on during the day
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The day to open as YYYY-MM-DD, defaults to today
    #[arg(short)]
    day: Option<String>,

    #[arg(short)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Read entries and print a summary
    Read {},
    /// Write a new entry
    Write {
        /// Informal description of the task
        description: Option<String>,

        /// Flag the current task as an endpoint, its time will not be counted
        #[arg(short)]
        stop: bool,

        /// User-defined context, used to group tasks, defaults to description
        #[arg(short)]
        context: Option<String>,

        /// The time of added line, as HH:MM, defaults to now
        #[arg(short)]
        time: Option<SimpleTime>,

        /// Indicate that the current task is minor, its duration will be distributed among other tasks
        #[arg(short)]
        minor: bool,

        /// Resume the last n-th task, a value of 0 resumes the first task, 1 the second.
        /// A negative value starts from the back, -1 resumes the current task, -2 the task before that.
        /// The time field will not be taken from the resumed task.
        #[arg(short)]
        resume: Option<i32>,
    },
}

/// Simple representation of the time of day
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct SimpleTime {
    hours: u32,
    minutes: u32,
}

impl FromStr for SimpleTime {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match NaiveTime::parse_from_str(s, "%H:%M") {
            Ok(t) => Ok(SimpleTime { hours: t.hour(), minutes: t.minute() }),
            Err(error) => Err(error)
        }
    }
}

fn today_str() -> String {
    format!("{}", chrono::offset::Local::now().format("%Y-%m-%d"))
}

enum TaskType {
    /// The entry is a regular task
    Regular,
    /// The entry terminates the previous task without starting a new one
    Stop,
    /// The entry is a minor task, its duration will be distributed among regular tasks
    Minor,
}

#[derive(Debug, Serialize, Deserialize)]
struct CsvRecord {
    description: String,
    start_time: SimpleTime,
    context: String,
    task_type: TaskType,
}

#[derive(Debug, Clone)]
struct HomeNotFoundError;

fn read_entries(filepath: &Path) -> Vec<CsvRecord> {
    if !filepath.exists() { return Vec::new(); }

    let mut reader =
        match csv::Reader::from_path(filepath) {
            Ok(reader) => { reader }
            Err(why) => panic!("couldn't read {:?}: {}", filepath, why),
        };


    let records = reader.deserialize()
        .collect::<Result<Vec<CsvRecord>, csv::Error>>()?;
}

fn write_entries(data: &Vec<CsvRecord>, filepath: &Path) {}

fn print_entries(data: &Vec<CsvRecord>) {}

fn main() {
    let args = Cli::parse();

    if args.verbose {
        println!("Args: {args:?}");
    }

    let day = args.day.unwrap_or(today_str());
    let filename = format!("{day}.csv");
    let home = dirs::home_dir().unwrap().join("time_tracker_rs");

    let filepath = home.join(filename);
    let actual_filepath = filepath.as_path();

    let mut entries: Vec<CsvRecord> = read_entries(actual_filepath);

    entries.sort_by(|a, b| a.start_time.cmp(&b.start_time))

    match args.command {
        Commands::Read => { print_entries(&entries); }
        write @ Commands::Write
        => {write.}
    }
}
