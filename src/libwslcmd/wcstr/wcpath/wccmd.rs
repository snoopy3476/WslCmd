// crate
use super::*;

// general
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

// consts
/// Extension of Windows binary
const EXECUTABLE_EXTENSION: &str = "exe";
/// Detached process prefix on cmdname
const DETACHED_PROC_PREFIX: char = '.';

// traits

/// Trait for extending WCCmd
pub trait WCCmd: WCCmdBase + Clone {}
// types for the trait (excluding from ancestors)

/// WslCmd path
pub trait WCCmdBase: WCPathBase + Clone {
    /// Convert wslcmd to filename with ext
    fn wccmd_with_ext(&self) -> Option<PathBuf> {
        self.wcpath_as_path().and_then(|p| {
            Some(p.with_file_name(format!("{}.{}", self.wcstr_as_str()?, EXECUTABLE_EXTENSION)))
        })
    }

    /// check if executable
    fn wccmd_is_exe(&self) -> bool {
        self.wcpath_as_path()
            .and_then(Path::extension)
            .and_then(OsStr::to_str)
            .map_or(false, |s| s == EXECUTABLE_EXTENSION)
    }

    /// Convert wslcmd (or its fname) to detached wslcmd (or its fname)
    fn wccmd_to_detached(&self) -> Option<PathBuf> {
        self.wcpath_as_path().and_then(|p| {
            Some(p.with_file_name(format!("{}{}", DETACHED_PROC_PREFIX, self.wcstr_as_str()?)))
        })
    }

    /// Check if detached wslcmd
    fn wccmd_is_detached(&self) -> bool {
        self.wcpath_fname()
            .map_or(false, |s| s.starts_with(DETACHED_PROC_PREFIX))
    }
}

impl WCCmdBase for String {}
impl WCCmdBase for Option<String> {}
impl WCCmdBase for &String {}
impl WCCmdBase for Option<&String> {}
impl WCCmdBase for &str {}
impl WCCmdBase for Option<&str> {}

/// impl for ancestor traits
mod ancestor_traits {
    use super::*;
    use std::path::{Path, PathBuf};

    ///// [`WCPath`] implementations for [`WCStr`]
    //impl<T: WCCmd> WCPathBase for T {}

    /// [`WCStr`] implementations for [`WCPath`]
    impl<T: WCPath> WCCmdBase for T {}
}
