// std lib
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek};
use std::path::Path;

// own
use crate::utils::{format_log, LogLevel, StrExt};

pub fn execute(arg: Option<&str>, header: Option<String>) -> Result<(), String> {
    // default arg
    let arg = arg.unwrap_or("all");

    // open files and initialze readers, writers
    let file_path = Path::new("dirs.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .map_err(|e| {
            format_log(
                LogLevel::Error,
                format!("Failed to open file \"dirs.txt\": {}", e),
            )
        })?;
    let line_count = BufReader::new(&file)
        .lines()
        .filter_map(|line| line.ok())
        .count();

    // reset cursor so we can read again
    _ = file.seek(std::io::SeekFrom::Start(0));
    let lines = BufReader::new(&file).lines().filter_map(|line| line.ok());

    let header = header.unwrap_or(format!("dirs.txt: length of {}", line_count));
    println!("----------------------------------------------------");
    println!("| {}", header);
    println!("----------------------------------------------------");
    let line_pad = 2;

    if arg == "all" || arg == "" {
        lines
            .enumerate()
            .for_each(|(i, line)| println!("| {:0>line_pad$} | {}", i + 1, line));
    } else if arg.chars().all(|char| char.is_digit(10)) {
        let line_num: usize = arg
            .parse()
            .map_err(|e| format_log(LogLevel::Error, format!("Failed to parse arg: {}", e)))?;

        lines
            .enumerate()
            .filter(|(i, _)| i + 1 == line_num)
            .for_each(|(i, line)| println!("| {:0>line_pad$} | {}", i + 1, line));
    } else if arg.is_digit_range() {
        let parts: Vec<&str> = arg.split('-').collect();
        let start = parts[0].parse::<usize>().unwrap();
        let end = parts[1].parse::<usize>().unwrap();

        if start < 1 {
            return Err(format_log(
                LogLevel::Error,
                format!("min is 1; got starting range {}", start),
            ));
        } else if end > line_count {
            return Err(format_log(
                LogLevel::Error,
                format!("max is {}; got ending range {}", line_count, end),
            ));
        } else if start > end {
            return Err(format_log(
                LogLevel::Error,
                format!(
                    "starting range {} is greater than ending range {}",
                    start, end
                ),
            ));
        } else if start == end {
            return Err(format_log(
                LogLevel::Error,
                format!("starting range {} is equal to ending range {}", start, end),
            ));
        }

        lines
            .enumerate()
            .filter(|(i, _)| i + 1 >= start && i + 1 <= end)
            .for_each(|(i, line)| println!("| {:0>line_pad$} | {}", i + 1, line));
    } else {
        let pattern = &arg.to_lowercase();

        lines
            .enumerate()
            .filter(|(_, line)| {
                line.to_lowercase().contains(pattern) || line.to_lowercase().contains(pattern)
            })
            .for_each(|(i, line)| {
                println!("| {:0>line_pad$} | {}", i + 1, line);
            })
    }

    println!("----------------------------------------------------");
    Ok(())
}

pub fn help(verbose: bool) {
    todo!()
}
