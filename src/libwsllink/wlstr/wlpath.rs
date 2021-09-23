use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// WslLink path
pub trait WLPath: Clone {
    ///// Get string reference inside WLPath
    //fn wlpath_ref(&self) -> Option<&str>;
    /// Return [`Path`] containing refered str slice inside [`WLPath`]
    fn wlpath_as_path(&self) -> Option<&Path>;

    /// Get ownership of [`WLPath`] if owned,
    /// or create new [`PathBuf`] with [`WLPath`] if refered,
    /// then return it
    fn wlpath_to_pathbuf(self) -> Option<PathBuf>;

    /// Get basename of [`WLPath`]
    fn wlpath_basename(&self) -> Option<&str> {
        self.wlpath_as_path().and_then(|p| {
            p.file_stem() // .. -> Option<&OsStr> basename
                .and_then(OsStr::to_str) // .. -> Option<&str>
        })
    }

    /// Follow and resolve all links of [`WLPath`]
    fn wlpath_canonicalize(&self) -> Option<PathBuf> {
        self.wlpath_as_path().and_then(|p| {
            p.canonicalize() // .. -> Result<PathBuf> link_resolved_path
                .ok() // .. -> Option<PathBuf>
        })
    }

    /// Check if [`WLPath`] is absolute path
    fn wlpath_is_absolute(&self) -> bool {
        self.wlpath_as_path().map_or(false, |p| p.is_absolute())
    }

    /// Get parent of [`WLPath`]
    fn wlpath_parent(&self) -> Option<&Path> {
        self.wlpath_as_path().and_then(|p| p.parent())
    }

    /// Read [`WLPath`] entries if directory
    ///
    /// Returns None if reading directory failed
    fn wlpath_read_dir(&self) -> Option<Vec<PathBuf>> {
        self.wlpath_as_path().and_then(|p| {
            Some(
                p.read_dir()
                    .ok()? // return None if read_dir fails
                    .filter_map(|res| res.as_ref().map(std::fs::DirEntry::path).ok())
                    .collect(),
            )
        })
    }
}

/// [`String`] implementations for [`WLPath`]
impl WLPath for String {
    fn wlpath_as_path(&self) -> Option<&Path> {
        Some(Path::new(self))
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(PathBuf::from(self))
    }
}
/// [`Option`]<[`String`]> implementations for [`WLPath`]
impl WLPath for Option<String> {
    fn wlpath_as_path(&self) -> Option<&Path> {
        self.as_deref().map(Path::new)
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// &[`String`] implementations for [`WLPath`]
impl WLPath for &String {
    fn wlpath_as_path(&self) -> Option<&Path> {
        Some(Path::new(self))
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(PathBuf::from(self))
    }
}
/// [`Option`]<&[`String`]> implementations for [`WLPath`]
impl WLPath for Option<&String> {
    fn wlpath_as_path(&self) -> Option<&Path> {
        self.map(Path::new)
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// &[`str`] implementations for [`WLPath`]
impl WLPath for &str {
    fn wlpath_as_path(&self) -> Option<&Path> {
        Some(Path::new(self))
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(PathBuf::from(self))
    }
}
/// [`Option`]<&[`str`]> implementations for [`WLPath`]
impl WLPath for Option<&str> {
    fn wlpath_as_path(&self) -> Option<&Path> {
        self.map(Path::new)
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// &[`Path`] implementations for [`WLPath`]
impl WLPath for &Path {
    fn wlpath_as_path(&self) -> Option<&Path> {
        Some(self)
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(self.to_path_buf())
    }
}
/// [`Option`]<&[`Path`]> implementations for [`WLPath`]
impl WLPath for Option<&Path> {
    fn wlpath_as_path(&self) -> Option<&Path> {
        *self
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// [`PathBuf`] implementations for [`WLPath`]
impl WLPath for PathBuf {
    fn wlpath_as_path(&self) -> Option<&Path> {
        Some(self.as_path())
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(self)
    }
}
/// [`Option`]<[`PathBuf`]> implementations for [`WLPath`]
impl WLPath for Option<PathBuf> {
    fn wlpath_as_path(&self) -> Option<&Path> {
        self.as_deref()
    }
    fn wlpath_to_pathbuf(self) -> Option<PathBuf> {
        self // return self (do nothing)
    }
}
