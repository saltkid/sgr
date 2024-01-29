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
