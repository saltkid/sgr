// std lib
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek};

// own
use crate::utils::{format_log, LogLevel, StrExt, StringExt};

pub fn execute(arg: Option<&str>, header: Option<String>) -> Result<(), String> {
    // default arg
    let arg = arg.unwrap_or("all");

    // open files and initialze readers, writers
    let file_path = std::env::current_exe()
        .map_err(|e| {
            format_log(
                LogLevel::Error,
                format!("Failed to get sgr.exe path: {}", e),
            )
        })?
        .parent()
        .ok_or("Failed to get parent directory of sgr.exe")?
        .join("dirs.txt");
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

    let header = header.unwrap_or(format!("({}) {:?}", line_count, file_path));
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
    let title = match verbose {
        true => "list"
            .to_string()
            .pad_right(15)
            .fill_left(2)
            .bold()
            .underline(),
        false => "list".to_string().pad_right(15).fill_left(2).bold(),
    };

    println!("{}{}\n", title, "prints all directories in dirs.txt.");
    if verbose {
        println!(
            "{}{}",
            "".to_string().pad_right(15).fill_left(2),
            "User can specify which directory or directories to print by"
        );
        println!(
            "{}{}\n",
            "".to_string().pad_right(15).fill_left(2),
            "specifying a line number or a range of line numbers respectively"
        );
        println!(
            "{}{}",
            "".to_string().pad_right(15).fill_left(2),
            "User can filter directories by specific pattern too"
        );
        println!(
            "{}{}",
            "".to_string().pad_right(15).fill_left(2),
            "Example: \"sgr list rust\" will print all directories with \"rust\" in them"
        );
        println!("\n{}", "Usage:".to_string().bold().underline().fill_left(2));
        println!("{}", "sgr list 1".to_string().fill_left(17));
        println!("{}", "sgr list 1-3".to_string().fill_left(17));
        println!("{}", "sgr list path/pattern".to_string().fill_left(17));

        println!("\n{}:", "Notes".to_string().bold().underline().fill_left(2));
        println!("  1. Line number bounds are checked");
        println!("  2. Line range is inclusive");
        println!("  3. Pattern is case insensitive but exact match is required");
        println!("     if none was found, nothing will be printed");
    }
}
