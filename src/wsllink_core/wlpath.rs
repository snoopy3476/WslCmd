use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// WslLink path
pub trait WLPath {
    /// Get string reference inside WLPath
    fn get_ref(&self) -> Option<&str>;

    /// Get ownership of WLPath if owned,
    /// or create new String with WLPath if refered,
    /// then return it
    fn get_owned(self) -> Option<String>;

    /// invoke function
    fn invoke_chain<T: WLPath>(&self, f: &dyn Fn(&str) -> T) -> Option<String> {
        self.get_ref().and_then(|s| f(s).get_owned())
    }

    /// do replace_all on self
    fn replace_all_regex(&self, match_pattern: &str, replace_with: &str) -> Option<String> {
        self.get_ref().and_then(|s| {
            regex::Regex::new(match_pattern)
                .map(|re| re.replace_all(&s, replace_with).into_owned())
                .ok()
        })
    }

    /// get basename of self
    fn path_basename(&self) -> Option<&str> {
        self.get_ref().and_then(|s| {
            Path::new(s) // Path (tmp module for filename process)
                .file_stem() // .. -> Option<&OsStr> basename
                .and_then(OsStr::to_str) // .. -> Option<&str>
        })
    }
}

// String
impl WLPath for String {
    fn get_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn get_owned(self) -> Option<String> {
        Some(self)
    }
}
impl WLPath for Option<String> {
    fn get_ref(&self) -> Option<&str> {
        self.as_deref()
    }
    fn get_owned(self) -> Option<String> {
        self // return self (do nothing)
    }
}

// &String
impl WLPath for &String {
    fn get_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn get_owned(self) -> Option<String> {
        Some(String::from(self))
    }
}
impl WLPath for Option<&String> {
    fn get_ref(&self) -> Option<&str> {
        self.map(String::as_str)
    }
    fn get_owned(self) -> Option<String> {
        self.map(String::from)
    }
}

// &str
impl WLPath for &str {
    fn get_ref(&self) -> Option<&str> {
        Some(self)
    }
    fn get_owned(self) -> Option<String> {
        Some((*self).to_string())
    }
}
impl WLPath for Option<&str> {
    fn get_ref(&self) -> Option<&str> {
        *self // return self (do nothing)
    }
    fn get_owned(self) -> Option<String> {
        self.map(String::from)
    }
}

// PathBuf
impl WLPath for PathBuf {
    fn get_ref(&self) -> Option<&str> {
        self.to_str()
    }
    fn get_owned(self) -> Option<String> {
        self.into_os_string().into_string().ok()
    }
}
impl WLPath for Option<PathBuf> {
    fn get_ref(&self) -> Option<&str> {
        self.as_deref().and_then(Path::to_str)
    }
    fn get_owned(self) -> Option<String> {
        self.and_then(|s| s.into_os_string().into_string().ok())
    }
}
