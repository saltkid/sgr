use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.len() < 2 {
        let res = run();
        match res {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
        return;
    }

    let command = &raw_args[1];
    let args = &raw_args[2..];
    if args.is_empty() {
        eprintln!("no args given for '{command}'");
        return;
    }

    if args.len() > 1 {
        eprintln!("args '{:?}' will be unused", args[1..].to_vec());
    }

    let arg = &args[0].to_string();
    let res = match command.as_str() {
        "add" => add(arg),
        "remove" => remove(arg),
        "list" => list(arg),
        _ => Err("unknown command".to_string()),
    };

    match res {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn run() -> Result<(), String> {
    let file = OpenOptions::new()
        .read(true)
        .open("dirs.txt")
        .map_err(|e| format!("Failed to open file \"dirs.txt\": {}", e))?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            }
            Err(e) => {
                return Err(format!("Failed to read line: {}", e));
            }
        }
    }

    Ok(())
}

fn add(dir: &String) -> Result<(), String> {
    let abs_path = Path::new(&dir)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

    if !abs_path.is_dir() {
        return Err(format!("\"{}\" is not a directory", abs_path.display()));
    }

    // TODO: check if abs_path contains any .git subdir; do walkdir

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("dirs.txt")
        .map_err(|e| format!("Failed to open file \"dirs.txt\": {}", e))?;

    let reader = BufReader::new(&file);
    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if line == abs_path.display().to_string() {
                    return Err(format!(
                        "\"{}\" already exists in dirs.txt on line {}",
                        abs_path.display(),
                        i + 1
                    ));
                }
            }
            Err(e) => {
                return Err(format!(
                    "Failed to read \"dirs.txt\" at line {}: {}",
                    i + 1,
                    e
                ));
            }
        }
    }

    let to_write = abs_path
        .display()
        .to_string()
        .strip_prefix(r#"\\?\"#)
        .unwrap_or(&abs_path.display().to_string())
        .to_string();

    if let Err(e) = writeln!(file, "{}", to_write) {
        return Err(format!("Failed to write to file: {}", e));
    }

    println!("{} added to dirs.txt", to_write);
    Ok(())
}

fn remove(args: &String) -> Result<(), String> {
    println!("ran remove with {:?}", args);
    return Ok(());
}

fn list(args: &String) -> Result<(), String> {
    println!("ran list with {:?}", args);
    return Ok(());
}
