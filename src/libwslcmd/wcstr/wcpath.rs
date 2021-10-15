use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// WslCmd path
pub trait WCPath: Clone {
    /// Return [`Path`] containing refered str slice inside [`WCPath`]
    fn wcpath_as_path(&self) -> Option<&Path>;

    /// Get ownership of [`WCPath`] if owned,
    /// or create new [`PathBuf`] with [`WCPath`] if refered,
    /// then return it
    fn wcpath_to_pathbuf(self) -> Option<PathBuf>;

    /// Clone new [`WCPath`] from self ref,
    /// then return it in form of [`PathBuf`]
    fn wcpath_clone_to_pathbuf(&self) -> Option<PathBuf> {
        self.clone().wcpath_to_pathbuf()
    }

    /// Get [`str`] reference of [`WCPath`]
    fn wcpath_as_ref(&self) -> Option<&str> {
        self.wcpath_as_path().and_then(Path::to_str)
    }

    /// Get basename of [`WCPath`]
    fn wcpath_basename(&self) -> Option<&str> {
        self.wcpath_as_path().and_then(|p| {
            p.file_stem() // .. -> Option<&OsStr> basename
                .and_then(OsStr::to_str) // .. -> Option<&str>
        })
    }

    /// Get filename of [`WCPath`]
    fn wcpath_filename(&self) -> Option<&str> {
        self.wcpath_as_path().and_then(|p| {
            p.file_name() // .. -> Option<&OsStr> basename
                .and_then(OsStr::to_str) // .. -> Option<&str>
        })
    }

    /// Follow and resolve all links of [`WCPath`]
    fn wcpath_canonicalize(&self) -> Option<PathBuf> {
        self.wcpath_as_path().and_then(|p| {
            p.canonicalize() // .. -> Result<PathBuf> link_resolved_path
                .ok() // .. -> Option<PathBuf>
        })
    }

    /// Check if [`WCPath`] is absolute path
    fn wcpath_is_absolute(&self) -> bool {
        self.wcpath_as_path().map_or(false, |p| p.is_absolute())
    }

    /// Get parent of [`WCPath`]
    fn wcpath_parent(&self) -> Option<&Path> {
        self.wcpath_as_path().and_then(|p| p.parent())
    }

    /// Read [`WCPath`] entries if directory
    ///
    /// Returns None if reading directory failed
    fn wcpath_read_dir(&self) -> Option<Vec<PathBuf>> {
        self.wcpath_as_path().and_then(|p| {
            Some(
                p.read_dir()
                    .ok()? // return None if read_dir fails
                    .filter_map(|res| res.as_ref().map(std::fs::DirEntry::path).ok())
                    .collect(),
            )
        })
    }
}

/// [`String`] implementations for [`WCPath`]
impl WCPath for String {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(Path::new(self))
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(PathBuf::from(self))
    }
}
/// [`Option`]<[`String`]> implementations for [`WCPath`]
impl WCPath for Option<String> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        self.as_deref().map(Path::new)
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// &[`String`] implementations for [`WCPath`]
impl WCPath for &String {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(Path::new(self))
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(PathBuf::from(self))
    }
}
/// [`Option`]<&[`String`]> implementations for [`WCPath`]
impl WCPath for Option<&String> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        self.map(Path::new)
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// &[`str`] implementations for [`WCPath`]
impl WCPath for &str {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(Path::new(self))
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(PathBuf::from(self))
    }
}
/// [`Option`]<&[`str`]> implementations for [`WCPath`]
impl WCPath for Option<&str> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        self.map(Path::new)
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// &[`Path`] implementations for [`WCPath`]
impl WCPath for &Path {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(self)
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(self.to_path_buf())
    }
}
/// [`Option`]<&[`Path`]> implementations for [`WCPath`]
impl WCPath for Option<&Path> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        *self
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// [`PathBuf`] implementations for [`WCPath`]
impl WCPath for PathBuf {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(self.as_path())
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(self)
    }
}
/// [`Option`]<[`PathBuf`]> implementations for [`WCPath`]
impl WCPath for Option<PathBuf> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        self.as_deref()
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self // return self (do nothing)
    }
}

/// &[`PathBuf`] implementations for [`WCPath`]
impl WCPath for &PathBuf {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(self.as_path())
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(self.clone())
    }
}
/// [`Option`]<&[`PathBuf`]> implementations for [`WCPath`]
impl WCPath for Option<&PathBuf> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        self.map(|pb| pb.as_path())
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(|pb| pb.clone())
    }
}
