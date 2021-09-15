/// Manages WslLink path
pub mod wlpath;
/// Converts Windows cmdline to WSL cmdline
pub mod wslcmd;

// prevent compilation at environments other than Windows
#[cfg(not(target_os = "windows"))]
compile_error!("WslLink only works on Windows environment!");
