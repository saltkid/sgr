use std::path::{Path, PathBuf};

pub trait PathExt {
    fn must_be_dir(&self) -> Result<PathBuf, String>;
}

impl PathExt for Path {
    fn must_be_dir(&self) -> Result<PathBuf, String> {
        if !self.is_dir() {
            let abs_path = self.display().to_string();
            let trimmed_path = abs_path.strip_prefix(r#"\\?\"#).unwrap_or(&abs_path);

            return Err(format_log(
                LogLevel::Error,
                format!("\"{}\" is not a directory", trimmed_path),
            ));
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
    println!("{} {}", log_header(level), msg);
}

pub fn format_log(level: LogLevel, msg: String) -> String {
    format!("{} {}", log_header(level), msg)
}

pub fn log_header(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Info => info_header(),
        LogLevel::Error => error_header(),
        LogLevel::Warn => warn_header(),
    }
}

fn info_header() -> &'static str {
    "[INFO]"
}

fn error_header() -> &'static str {
    "\x1b[31m[ERROR]\x1b[0m"
}

fn warn_header() -> &'static str {
    "\x1b[33m[WARN]\x1b[0m"
}
