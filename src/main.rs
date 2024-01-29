// standard library
use std::fs::{remove_file, rename, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// third party
use walkdir::WalkDir;

// own
mod utils;
use utils::*;

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.len() < 2 {
        match run() {
            Ok(path) => println!("{}", path),
            Err(e) => eprintln!("{}", e),
        }
        return;
    }

    let command = &raw_args[1];
    match command.as_str() {
        "add" | "remove" | "list" => {}
        _ => {
            eprintln!("unknown command '{}'", command);
            return;
        }
    };

    let args = &raw_args[2..];
    if args.is_empty() {
        eprintln!("no args given for '{}'", command);
        return;
    }

    if args.len() > 1 {
        eprintln!("args '{:?}' will be unused", args[1..].to_vec());
    }

    let arg = &args[0];
    let res = match command.as_str() {
        "add" => add(arg),
        "remove" => remove(arg),
        "list" => list(arg),
        _ => Err(format!("unknown command '{}'", command)),
    };
    match res {
        Ok(_) => println!("done"),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<String, String> {
    let file = OpenOptions::new()
        .read(true)
        .open("dirs.txt")
        .map_err(|e| format!("Failed to open file \"dirs.txt\": {}", e))?;

    let reader = BufReader::new(file);
    let lines: Vec<PathBuf> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| PathBuf::from(line))
        .collect();

    let mut fzf_process = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to start fzf: {}", e))?;

    if let Some(stdin) = fzf_process.stdin.as_mut() {
        for path in lines {
            for e in WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_dir() && e.path().ends_with(".git"))
            {
                let git_repo = e.path().parent().unwrap_or(e.path());
                writeln!(stdin, "{}", git_repo.display())
                    .map_err(|e| format!("Failed to write to stdin: {}", e))?;
            }
        }
    }

    let output = fzf_process
        .wait_with_output()
        .map_err(|e| format!("failed to wait for fzf: {}", e))?;

    if !output.status.success() {
        return Err(format!("Did not choose a directory"));
    }

    let selected_path = String::from_utf8_lossy(&output.stdout).to_string();
    return Ok(selected_path);
}

fn add(dir: &str) -> Result<(), String> {
    let abs_path = Path::new(&dir)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {}", e))?
        .must_be_dir()?
        .display()
        .to_string();

    let trimmed_path = abs_path.strip_prefix(r#"\\?\"#).unwrap_or(&abs_path);

    WalkDir::new(&trimmed_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir() && e.path().ends_with(".git"))
        .count()
        .gt(&0)
        .then(|| ())
        .ok_or(format!(
            "No git repos found in directory '{}'",
            trimmed_path,
        ))?;

    let file_path = Path::new("dirs.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(&file_path)
        .map_err(|e| format!("Failed to open file \"dirs.txt\": {}", e))?;

    if BufReader::new(&file)
        .lines()
        .filter_map(|line| line.ok())
        .any(|line| line.trim().eq_ignore_ascii_case(trimmed_path))
    {
        return Err(format!(
            "\"{}\" already exists in \"{}\"",
            trimmed_path,
            file_path.display()
        ));
    }

    if let Err(e) = writeln!(file, "{}", trimmed_path) {
        return Err(format!("Failed to write to file: {}", e));
    }

    println!("{} added to dirs.txt", trimmed_path);
    Ok(())
}

fn remove(arg: &str) -> Result<(), String> {
    // open files and initialze readers, writers
    let file_path = Path::new("dirs.txt");
    let file = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .map_err(|e| format!("Failed to open file \"dirs.txt\": {}", e))?;

    let temp_file_path = Path::new("temp_dirs.txt");
    let temp_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&temp_file_path)
        .map_err(|e| format!("Failed to create \"temp_dirs.txt\": {}", e))?;
    let mut writer = BufWriter::new(&temp_file);
    let lines = BufReader::new(&file).lines().filter_map(|line| line.ok());

    if arg.chars().all(|char| char.is_digit(10)) {
        let line_num: usize = arg
            .parse()
            .map_err(|e| format!("Unexpected error, unable to parse {} to int: {}", arg, e))?;

        lines
            .enumerate()
            .filter(|(i, _)| i + 1 != line_num)
            .try_for_each(|(_, line)| {
                writeln!(writer, "{}", line)
                    .map_err(|e| format!("Failed to write \"{}\" to temp_dirs.txt: {}", line, e))
            })?;
    } else if arg.is_digit_range() {
        let parts: Vec<&str> = arg.split('-').collect();
        let start = parts[0].parse::<usize>().unwrap();
        let end = parts[1].parse::<usize>().unwrap();

        lines
            .enumerate()
            .filter(|(i, _)| i + 1 < start || i + 1 > end)
            .try_for_each(|(_, line)| {
                writeln!(writer, "{}", line)
                    .map_err(|e| format!("Failed to write \"{}\" to temp_dirs.txt: {}", line, e))
            })?;
    } else {
        let abs_path = Path::new(&arg)
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize path {}: {}", &arg, e))?
            .must_be_dir()?
            .display()
            .to_string();
        let trimmed_path = abs_path.strip_prefix(r#"\\?\"#).unwrap_or(&abs_path);

        lines
            .filter(|line| !line.trim().eq_ignore_ascii_case(&trimmed_path))
            .try_for_each(|line| {
                writeln!(writer, "{}", line)
                    .map_err(|e| format!("Failed to write \"{}\" to temp_dirs.txt: {}", line, e))
            })?;
    }

    // flush and rename
    writer
        .flush()
        .map_err(|e| format!("Failed to flush temp_dirs.txt: {}", e))?;

    remove_file(&file_path).map_err(|e| format!("Failed to remove file \"dirs.txt\": {}", e))?;
    rename(&temp_file_path, &file_path)
        .map_err(|e| format!("failed to rename \"temp_dirs.txt\" to \"dirs.txt\": {}", e))?;

    Ok(())
}

fn list(args: &str) -> Result<(), String> {
    println!("ran list with {:?}", args);
    return Ok(());
}
