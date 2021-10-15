/// Manages WslCmd str
pub mod wcstr;

/// Converts Windows cmdline to WSL cmdline
mod wslcmd;
pub use wslcmd::WslCmd;

/// Manage list of WslCmd in a directory
mod wslcmd_list;
pub use wslcmd_list::WslCmdList;

// prevent compilation at environments other than Windows
#[cfg(not(target_os = "windows"))]
compile_error!("WslCmd only works on Windows target!");
