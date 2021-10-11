/// Manages WslLink str
mod wlstr;
pub use wlstr::WLPath;
pub use wlstr::WLStr;
/// Converts Windows cmdline to WSL cmdline
mod wslcmd;
pub use wslcmd::WslCmd;
/// Manage list of WslCmd in a directory
mod wslcmd_list;
pub use wslcmd_list::WslCmdList;

/// Detached process prefix on cmdname
const DETACHED_PROC_PREFIX: char = '.';

// prevent compilation at environments other than Windows
#[cfg(not(target_os = "windows"))]
compile_error!("WslLink only works on Windows target!");
