// crate
use super::*;

// general
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

// traits

/// Trait for extending WCPath
pub trait WCPath: WCPathBase + Clone {}
// types for the trait (excluding from ancestors)
impl WCPath for &Path {}
impl WCPath for Option<&Path> {}
impl WCPath for PathBuf {}
impl WCPath for Option<PathBuf> {}
impl WCPath for &PathBuf {}
impl WCPath for Option<&PathBuf> {}

/// WslCmd path
pub trait WCPathBase: WCStrBase + Clone {
    /// Return [`Path`] containing refered str slice inside [`WCPathBase`]
    fn wcpath_as_path(&self) -> Option<&Path>;

    /// Get ownership of [`WCPathBase`] if owned,
    /// or create new [`PathBuf`] with [`WCPathBase`] if refered,
    /// then return it
    fn wcpath_to_pathbuf(self) -> Option<PathBuf>;

    /// Clone new [`WCPathBase`] from self ref,
    /// then return it in form of [`PathBuf`]
    fn wcpath_clone_to_pathbuf(&self) -> Option<PathBuf> {
        self.clone().wcpath_to_pathbuf()
    }

    /// Get [`str`] reference of [`WCPathBase`]
    /*fn wcpath_as_str(&self) -> Option<&str> {
        self.wcpath_as_path().and_then(Path::to_str)
    }*/

    /// Get basename of [`WCPathBase`]
    fn wcpath_fstem(&self) -> Option<&str> {
        self.wcpath_as_path().and_then(|p| {
            p.file_stem() // .. -> Option<&OsStr> basename
                .and_then(OsStr::to_str) // .. -> Option<&str>
        })
    }

    /// Get filename of [`WCPathBase`]
    fn wcpath_fname(&self) -> Option<&str> {
        self.wcpath_as_path().and_then(|p| {
            p.file_name() // .. -> Option<&OsStr> basename
                .and_then(OsStr::to_str) // .. -> Option<&str>
        })
    }

    /// Follow and resolve all links of [`WCPathBase`]
    fn wcpath_canonicalize(&self) -> Option<PathBuf> {
        self.wcpath_as_path().and_then(|p| {
            p.canonicalize() // .. -> Result<PathBuf> link_resolved_path
                .ok() // .. -> Option<PathBuf>
        })
    }

    /// Check if [`WCPathBase`] is absolute path
    fn wcpath_is_absolute(&self) -> bool {
        self.wcpath_as_path().map_or(false, |p| p.is_absolute())
    }

    /// Get parent of [`WCPathBase`]
    fn wcpath_parent(&self) -> Option<&Path> {
        self.wcpath_as_path().and_then(|p| p.parent())
    }

    /// Read [`WCPathBase`] entries if directory
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

/// &[`Path`] implementations for [`WCPathBase`]
impl WCPathBase for &Path {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(self)
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(self.to_path_buf())
    }
}
/// [`Option`]<&[`Path`]> implementations for [`WCPathBase`]
impl WCPathBase for Option<&Path> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        *self
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(PathBuf::from)
    }
}

/// [`PathBuf`] implementations for [`WCPathBase`]
impl WCPathBase for PathBuf {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(self.as_path())
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(self)
    }
}
/// [`Option`]<[`PathBuf`]> implementations for [`WCPathBase`]
impl WCPathBase for Option<PathBuf> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        self.as_deref()
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self // return self (do nothing)
    }
}

/// &[`PathBuf`] implementations for [`WCPathBase`]
impl WCPathBase for &PathBuf {
    fn wcpath_as_path(&self) -> Option<&Path> {
        Some(self.as_path())
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        Some(self.clone())
    }
}
/// [`Option`]<&[`PathBuf`]> implementations for [`WCPathBase`]
impl WCPathBase for Option<&PathBuf> {
    fn wcpath_as_path(&self) -> Option<&Path> {
        self.map(|pb| pb.as_path())
    }
    fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
        self.map(|pb| pb.clone())
    }
}

/// impl for ancestor traits
mod ancestor_traits {
    use super::*;
    use std::path::{Path, PathBuf};

    /// [`WCPath`] implementations for [`WCStr`]
    impl<T: WCPath> WCStrBase for T {
        fn wcstr_as_str(&self) -> Option<&str> {
            self.wcpath_as_path().and_then(Path::to_str)
        }

        fn wcstr_to_string(self) -> Option<String> {
            self.wcpath_as_path()
                .and_then(Path::to_str)
                .wcstr_clone_to_string()
        }
    }

    /// [`WCStr`] implementations for [`WCPath`]
    impl<T: WCStr> WCPathBase for T {
        fn wcpath_as_path(&self) -> Option<&Path> {
            self.wcstr_as_str().map(Path::new)
        }
        fn wcpath_to_pathbuf(self) -> Option<PathBuf> {
            self.wcstr_as_str().map(PathBuf::from)
        }
    }
}
