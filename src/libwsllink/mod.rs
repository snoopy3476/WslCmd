/// Manages WslLink str
mod wlstr;
pub use wlstr::WLPath;
pub use wlstr::WLStr;
/// Converts Windows cmdline to WSL cmdline
mod wslcmd;
pub use wslcmd::WslCmd;

// prevent compilation at environments other than Windows
#[cfg(not(target_os = "windows"))]
compile_error!("WslLink only works on Windows target!");
