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

pub trait StringExt {
    // text space padding
    fn pad_right(&self, len: usize) -> String;
    fn pad_left(&self, len: usize) -> String;
    fn pad_mid(&self, len: usize) -> String;
    fn fill_right(&self, len: usize) -> String;
    fn fill_left(&self, len: usize) -> String;

    // text formatting
    fn bold(&self) -> String;
    fn underline(&self) -> String;
    fn italic(&self) -> String;
}

impl StringExt for String {
    fn pad_right(&self, len: usize) -> String {
        let width = match len < self.len() {
            true => 0,
            false => len - self.len(),
        };
        return format!("{}{}", self, " ".repeat(width));
    }

    fn pad_left(&self, len: usize) -> String {
        let width = match len < self.len() {
            true => 0,
            false => len - self.len(),
        };
        return format!("{}{}", " ".repeat(width), self);
    }

    fn pad_mid(&self, len: usize) -> String {
        let width = match len < self.len() {
            true => 0,
            false => len - self.len(),
        };
        return format!(
            "{}{}{}",
            " ".repeat(width / 2),
            self,
            " ".repeat(width - width / 2)
        );
    }

    fn fill_right(&self, len: usize) -> String {
        return format!("{}{}", self, " ".repeat(len));
    }

    fn fill_left(&self, len: usize) -> String {
        return format!("{}{}", " ".repeat(len), self);
    }

    fn bold(&self) -> String {
        let (leading, string) = match self.find(|c: char| !c.is_whitespace()) {
            Some(idx) => self.split_at(idx),
            None => ("", self.as_str()),
        };

        let (string, trailing) = match string.rfind(|c: char| !c.is_whitespace()) {
            Some(idx) => string.split_at(idx + 1),
            None => (string, ""),
        };

        format!("{}\x1b[1m{}\x1b[0m{}", leading, string, trailing)
    }

    fn underline(&self) -> String {
        let (leading, string) = match self.find(|c: char| !c.is_whitespace()) {
            Some(idx) => self.split_at(idx),
            None => ("", self.as_str()),
        };

        let (string, trailing) = match string.rfind(|c: char| !c.is_whitespace()) {
            Some(idx) => string.split_at(idx + 1),
            None => (string, ""),
        };

        format!("{}\x1b[4m{}\x1b[0m{}", leading, string, trailing)
    }

    fn italic(&self) -> String {
        let (leading, string) = match self.find(|c: char| !c.is_whitespace()) {
            Some(idx) => self.split_at(idx),
            None => ("", self.as_str()),
        };

        let (string, trailing) = match string.rfind(|c: char| !c.is_whitespace()) {
            Some(idx) => string.split_at(idx + 1),
            None => (string, ""),
        };

        format!("{}\x1b[3m{}\x1b[0m{}", leading, string, trailing)
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
