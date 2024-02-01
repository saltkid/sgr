// std lib
use std::fs::{remove_file, rename, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Seek, Write};
use std::path::Path;

// own
use crate::list;
use crate::utils::{format_log, LogLevel, PathExt, StrExt};

pub fn execute(arg: Option<&str>) -> Result<(), String> {
    // before
    list::execute(Some("all"), Some("dirs.txt: before remove".to_string()))?;

    // need arg
    let arg = arg.ok_or(format_log(
        LogLevel::Error,
        "missing arg for 'remove'".to_string(),
    ))?;

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

    let temp_file_path = Path::new("temp_dirs.txt");
    let temp_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&temp_file_path)
        .map_err(|e| {
            format_log(
                LogLevel::Error,
                format!("Failed to open file \"temp_dirs.txt\": {}", e),
            )
        })?;
    let mut writer = BufWriter::new(&temp_file);
    let line_count = BufReader::new(&file)
        .lines()
        .filter_map(|line| line.ok())
        .count();
    _ = file.seek(std::io::SeekFrom::Start(0));
    let lines = BufReader::new(&file).lines().filter_map(|line| line.ok());

    let mut _header_arg = "".to_string();
    if arg.chars().all(|char| char.is_digit(10)) {
        let line_num: usize = arg
            .parse()
            .map_err(|e| format_log(LogLevel::Error, format!("Failed to parse arg: {}", e)))?;

        if line_num > line_count {
            return Err(format_log(
                LogLevel::Error,
                format!("max is {}; got {}", line_count, line_num),
            ));
        } else if line_num < 1 {
            return Err(format_log(
                LogLevel::Error,
                format!("min is 1; got {}", line_num),
            ));
        }

        lines
            .enumerate()
            .filter(|(i, _)| i + 1 != line_num)
            .try_for_each(|(_, line)| {
                writeln!(writer, "{}", line).map_err(|e| {
                    format_log(
                        LogLevel::Error,
                        format!("Failed to write \"{}\" to temp_dirs.txt: {}", line, e),
                    )
                })
            })?;

        _header_arg = format!("line {}", line_num);
    } else if arg.is_digit_range() {
        let parts: Vec<&str> = arg.split('-').collect();
        let start = parts[0].parse::<usize>().unwrap();
        let end = parts[1].parse::<usize>().unwrap();

        if start < 1 {
            return Err(format_log(
                LogLevel::Error,
                format!("min is 1; got {}", start),
            ));
        } else if end > line_count {
            return Err(format_log(
                LogLevel::Error,
                format!("max is {}; got {}", line_count, end),
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
            .filter(|(i, _)| i + 1 < start || i + 1 > end)
            .try_for_each(|(_, line)| {
                writeln!(writer, "{}", line).map_err(|e| {
                    format_log(
                        LogLevel::Error,
                        format!("Failed to write \"{}\" to temp_dirs.txt: {}", line, e),
                    )
                })
            })?;

        _header_arg = format!("lines {}-{}", start, end);
    } else {
        let abs_path = Path::new(&arg)
            .canonicalize()
            .map_err(|e| {
                format_log(
                    LogLevel::Error,
                    format!("Failed to canonicalize path {}: {}", &arg, e),
                )
            })?
            .must_be_dir()?
            .display()
            .to_string();
        let trimmed_path = abs_path.strip_prefix(r#"\\?\"#).unwrap_or(&abs_path);

        lines
            .filter(|line| !line.trim().eq_ignore_ascii_case(&trimmed_path))
            .try_for_each(|line| {
                writeln!(writer, "{}", line).map_err(|e| {
                    format_log(
                        LogLevel::Error,
                        format!("Failed to write \"{}\" to temp_dirs.txt: {}", line, e),
                    )
                })
            })?;

        _header_arg = format!("\"{}\"", trimmed_path);
    }

    // need to flush before closing writer
    writer
        .flush()
        .map_err(|e| format!("Failed to flush temp_dirs.txt: {}", e))?;

    remove_file(&file_path).map_err(|e| {
        format_log(
            LogLevel::Error,
            format!("Failed to remove file \"dirs.txt\": {}", e),
        )
    })?;
    rename(&temp_file_path, &file_path).map_err(|e| {
        format_log(
            LogLevel::Error,
            format!("Failed to rename \"temp_dirs.txt\" to \"dirs.txt\": {}", e),
        )
    })?;

    // list updated dir
    list::execute(Some("all"), Some(format!("removed: {}", _header_arg)))?;
    Ok(())
}

pub fn help(verbose: bool) {
    todo!()
}
