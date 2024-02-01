// std lib
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// third party
use walkdir::WalkDir;

//own
use crate::list;
use crate::utils::{format_log, LogLevel, PathExt};

pub fn execute(dir: Option<&str>) -> Result<(), String> {
    // before
    list::execute(Some("all"), Some("dirs.txt: before add".to_string()))?;

    // need arg
    let dir = dir.ok_or(format!("missing arg for 'add'"))?;

    let abs_path = Path::new(&dir)
        .canonicalize()
        .map_err(|e| {
            format_log(
                LogLevel::Error,
                format!("Failed to canonicalize path: {}", e),
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

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("dirs.txt")
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

            if line_lowercase.eq_ignore_ascii_case(&trimmed_path_lowercase) {
                collision_msg = format_log(
                    LogLevel::Error,
                    format!("collision: \"{}\" already exists", trimmed_path),
                );
                true
            } else if trimmed_path_lowercase.starts_with(&line_lowercase) {
                collision_msg = format_log(
                    LogLevel::Error,
                    format!(
                        "collision: \"{}\" is a sub dir of \"{}\"",
                        trimmed_path, line
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
    println!("\x1B[4m\x1B[1madd\x1B[0m    adds a directory to dirs.txt");
    println!(
        "       The directory can either be a git repo or a directory that contains git repos"
    );
    println!("\n\x1B[4m\x1B[1mUsage\x1B[0m: sgr add <path/to/dir>");

    if verbose {
        println!("\n\x1B[4m\x1B[1mNotes\x1B[0m:");
        println!("  1. If the directory to be added is not a git repo, it must have subdirs that are git repos (it will be checked)\n");
        println!("  2. The directory to be added must NOT be a parent or a subdir of another directory in dirs.txt");
        println!("     This is because sgr already searches recursively for git repos for every directory in dirs.txt");
        println!(
            "     That means double the work for the same result so \"add\" just disallows that\n"
        );
        println!("  3. The directory to be added must already exist");
        println!("     sgr will not create directories for you");
    }
}
