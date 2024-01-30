use std::path::{Path, PathBuf};

pub trait PathExt {
    fn must_be_dir(&self) -> Result<PathBuf, String>;
}

impl PathExt for Path {
    fn must_be_dir(&self) -> Result<PathBuf, String> {
        if !self.is_dir() {
            return Err(format!("\"{}\" is not a directory", self.display()));
        }
        Ok(self.to_path_buf())
    }
}

pub trait StrExt {
    fn is_digit_range(&self) -> bool;
}

impl StrExt for str {
    fn is_digit_range(&self) -> bool {
        let parts: Vec<&str> = self.split('-').collect();

        if parts.len() != 2 {
            return false;
        }

        let start = parts[0].parse::<u32>();
        let end = parts[1].parse::<u32>();

        if start.is_err() || end.is_err() {
            return false;
        }

        return true;
    }
}

pub enum LogLevel {
    Info,
    Error,
    Warn,
}

pub fn logln(level: LogLevel, msg: String) {
    match level {
        LogLevel::Info => println!("[INFO]: {}", msg),
        LogLevel::Error => println!("\x1b[31m[ERROR]\x1b[0m: {}", msg),
        LogLevel::Warn => println!("\x1b[33m[WARN]\x1b[0m: {}", msg),
    };
}
