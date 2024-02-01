// std lib
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// third party
use walkdir::WalkDir;

//own
use crate::list;
use crate::utils::{format_log, LogLevel, PathExt, StringExt};

pub fn execute(dir: Option<&str>) -> Result<(), String> {
    // before
    list::execute(Some("all"), Some("dirs.txt: before add".to_string()))?;

    // need arg
    let dir = dir.ok_or(format_log(
        LogLevel::Error,
        "missing arg for 'add'".to_string(),
    ))?;

    let abs_path = Path::new(&dir)
        .canonicalize()
        .map_err(|e| {
            format_log(
                LogLevel::Error,
                format!("Failed to canonicalize path {}: {}", &dir, e),
            )
        })?
        .must_be_dir()?
        .display()
        .to_string();

    let trimmed_path = abs_path.strip_prefix(r#"\\?\"#).unwrap_or(&abs_path);

    WalkDir::new(&trimmed_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| {
            e.file_type().is_dir() && e.path().ends_with(".git") && e.path().join("HEAD").exists()
        })
        .ok_or(format_log(
            LogLevel::Error,
            format!("No git repos found in directory '{}'", trimmed_path),
        ))?;

    let dirs_txt_path = std::env::current_exe()
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
        .write(true)
        .append(true)
        .open(&dirs_txt_path)
        .map_err(|e| {
            format_log(
                LogLevel::Error,
                format!("Failed to open file \"dirs.txt\": {}", e),
            )
        })?;

    let mut collision_msg: String = "".to_string();
    if BufReader::new(&file)
        .lines()
        .filter_map(|line| line.ok())
        .any(|line| {
            let line_lowercase = line.trim().to_lowercase();
            let trimmed_path_lowercase = trimmed_path.to_lowercase();

            // already exists
            if line_lowercase.eq_ignore_ascii_case(&trimmed_path_lowercase) {
                collision_msg = format_log(
                    LogLevel::Error,
                    format!("collision: \"{}\" already exists", trimmed_path),
                );
                true
            // to be added is a subdir of existing
            } else if trimmed_path_lowercase.starts_with(&line_lowercase) {
                collision_msg = format_log(
                    LogLevel::Error,
                    format!(
                        "collision: \"{}\" is a sub dir of \"{}\"",
                        trimmed_path, line
                    ),
                );
                true
            // existing is a subdir of to be added
            } else if line_lowercase.starts_with(&trimmed_path_lowercase) {
                collision_msg = format_log(
                    LogLevel::Error,
                    format!(
                        "collision: \"{}\" is a sub dir of \"{}\"",
                        line, trimmed_path
                    ),
                );
                true
            } else {
                false
            }
        })
    {
        return Err(collision_msg);
    }

    if let Err(e) = writeln!(file, "{}", trimmed_path) {
        return Err(format_log(
            LogLevel::Error,
            format!("Failed to write to file: {}", e),
        ));
    }

    // list updated dir
    list::execute(Some("all"), Some(format!("added: {}", trimmed_path)))?;
    Ok(())
}

pub fn help(verbose: bool) {
    let title = match verbose {
        true => "add"
            .to_string()
            .pad_right(15)
            .fill_left(2)
            .bold()
            .underline(),
        false => "add".to_string().pad_right(15).fill_left(2).bold(),
    };

    println!("{}{}\n", title, "adds a directory to dirs.txt");
    if verbose {
        println!(
            "{}{}",
            "".to_string().pad_right(15).fill_left(2),
            "The directory can either be a git repo or a directory that contains git repos"
        );
        println!(
            "\n  {}{}",
            "Usage:".to_string().pad_right(15).bold().underline(),
            "sgr add path/to/dir".to_string()
        );

        println!("\n{}:", "Notes".to_string().fill_left(2).bold().underline());
        println!("  1. If the directory to be added is not a git repo, it must have subdirs that are git repos (it will be checked)");
        println!("  2. The directory to be added must NOT be a parent or a subdir of another directory in dirs.txt");
        println!("     This is because sgr already searches recursively for git repos for every directory in dirs.txt");
        println!(
            "     That means double the work for the same result so \"add\" just disallows that"
        );
        println!("  3. The directory to be added must already exist");
        println!("     sgr will not create directories for you");
    }
}
