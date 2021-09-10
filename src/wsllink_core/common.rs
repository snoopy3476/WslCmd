use std::path::Path;

pub fn file_basename(path: &str) -> &str {
    Path::new(path) // Path (tmp module for filename process)
        .file_stem() // .. -> Option<&OsStr> basename
        .map_or(None, std::ffi::OsStr::to_str) // .. -> Option<&str>
        .unwrap_or("") // .. -> &str
}
