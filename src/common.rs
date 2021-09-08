use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

pub trait PathTrait {
    fn basename(&self) -> Option<&OsStr>;
    fn deref_link(&self) -> Option<PathBuf>;
    fn to_path_buf(&self) -> PathBuf;
}

impl PathTrait for Path {
    fn basename(&self) -> Option<&OsStr> {
        self.file_stem()
    }
    fn deref_link(&self) -> Option<PathBuf> {
        self.read_link().ok()
    }
    fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(self)
    }
}

impl PathTrait for PathBuf {
    fn basename(&self) -> Option<&OsStr> {
        self.file_stem()
    }
    fn deref_link(&self) -> Option<PathBuf> {
        self.read_link().ok()
    }
    fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(self)
    }
}
