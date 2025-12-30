use clap::Parser;
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    io::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
enum Error {
    NotFoundHistFile,
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFoundHistFile => write!(f, "not found history file"),
            Error::Io(error) => write!(f, "io error: {}", error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'c', long = "count", help = "default: 10")]
    count: Option<i32>,
}

fn load_hist_path() -> Result<PathBuf, Error> {
    let home_dir = std::env::home_dir().unwrap();
    let mut history_files = fs::read_dir(&home_dir)?
        .filter_map(|f| f.ok())
        .map(|f| f.path())
        .filter(|f| {
            f.file_name()
                .map(|f| {
                    let n = f.to_string_lossy().to_string();
                    n.contains("history")
                        && (n.contains("zsh") || n.contains("bash") || n.contains("fish"))
                })
                .unwrap_or(false)
        })
        .map(|f| (f.clone(), f.metadata().and_then(|m| m.modified())))
        .filter_map(|f| match f.1 {
            Ok(v) => Some((f.0, v)),
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    history_files.sort_by_key(|f| f.1);
    let latest_history_path = history_files
        .last()
        .ok_or(Error::NotFoundHistFile)?
        .0
        .to_path_buf();

    Ok(latest_history_path)
}

fn load_hist<T: AsRef<Path>>(path: T) -> Result<Vec<String>, Error> {
    let bytes = fs::read(path)?;
    let content = String::from_utf8_lossy(&bytes).to_string();
    let lines = content
        .split('\n')
        .map(|f| f.to_string())
        .collect::<Vec<_>>();

    Ok(lines)
}

fn sort_command_counts(counts: HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut sorted_counts = counts.clone().into_iter().collect::<Vec<_>>();
    sorted_counts.sort_by_key(|f| f.1);
    sorted_counts
}

fn format_statistics(stats: Vec<(String, i32)>, count: i32) -> Vec<String> {
    let display_limit = if count > stats.len() as i32 {
        stats.len()
    } else {
        count as usize
    };

    let mut formatted_lines = stats
        .iter()
        .map(|f| format!("count: {}\t {}\n", f.1, f.0))
        .collect::<Vec<_>>();
    formatted_lines.reverse();
    let result = &formatted_lines[0..display_limit];
    result.to_vec()
}

fn count_commands(lines: Vec<String>) -> HashMap<String, i32> {
    lines
        .iter()
        .filter_map(|line| {
            if let Some((_, main)) = line.split_once(':') {
                if let Some((status_str, cmd)) = main.split_once(';') {
                    let status = status_str.parse::<i32>().unwrap_or(0);
                    let cmd = match cmd.split_once(' ') {
                        Some((first_token, rest_of_command)) => {
                            if first_token == "sudo" {
                                rest_of_command
                            } else {
                                first_token
                            }
                        }
                        None => cmd,
                    }
                    .to_string();
                    if status == 0 { Some(cmd) } else { None }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x.to_string()).or_insert(0) += 1;
            acc
        })
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let count = cli.count.unwrap_or(10);

    let hist_path = load_hist_path()?;
    println!("{hist_path:?}");
    let hist_lines = load_hist(&hist_path)?;

    let mut final_output =
        format_statistics(sort_command_counts(count_commands(hist_lines)), count);
    final_output.reverse();
    println!("{}", final_output.concat());
    Ok(())
}
