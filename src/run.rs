// standard library
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// third party
use walkdir::WalkDir;

// own
use crate::utils::{format_log, LogLevel};

pub fn execute() -> Result<String, String> {
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

    let file = OpenOptions::new()
        .read(true)
        .open(&dirs_txt_path)
        .map_err(|e| {
            format_log(
                LogLevel::Error,
                format!("Failed to open file \"{:?}\": {}", dirs_txt_path, e),
            )
        })?;

    let reader = BufReader::new(file);
    let mut lines = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| PathBuf::from(line));

    let mut fzf_process = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format_log(LogLevel::Error, format!("failed to start fzf: {}", e)))?;

    if let Some(stdin) = fzf_process.stdin.as_mut() {
        lines.try_for_each(|path| {
            WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_dir()
                        && e.path().ends_with(".git")
                        && e.path().join("HEAD").exists()
                })
                .try_for_each(|e| {
                    let git_repo = e.path().parent().unwrap_or(e.path());
                    writeln!(stdin, "{}", git_repo.display()).map_err(|e| {
                        format_log(LogLevel::Error, format!("Failed to write to stdin: {}", e))
                    })
                })
        })?
    }

    let output = fzf_process
        .wait_with_output()
        .map_err(|e| format_log(LogLevel::Error, format!("Failed to wait for fzf: {}", e)))?;

    if !output.status.success() {
        return Err(format_log(
            LogLevel::Warn,
            "Did not choose a directory".to_string(),
        ));
    }

    let selected_path = String::from_utf8_lossy(&output.stdout).to_string();
    return Ok(selected_path);
}
