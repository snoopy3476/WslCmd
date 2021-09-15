#![crate_type = "bin"]
#![crate_name = "wsllink"]

/// A module for executing WSL commands
mod execution_mode;
/// A module for managing WslLink itself
mod management_mode;

/// Core routines of WslLink
mod wsllink_core;
use wsllink_core::wlpath::WLPath;

/// Exporting macros for debug print
mod dbg_print;

/// Branch routine (Management/Execution mode),
/// by checking if the binary is executed directly or through symlink
fn main() {
    // debug msg
    crate::__wsllink_dbg!("* Executed in debug mode! Debug msgs will be printed. *");

    // init
    windows_freeconsole();
    let args: Vec<String> = std::env::args().collect();

    // call either mode, then get exitcode
    let exit_code: Option<i32> = match orig_binname() != cmdarg_binname(&args) {
        // if executed through command symlink
        true => execution_mode::execution_mode(&args),
        // if executed directly
        // (including symlinks of which basename == 'orig bin basename')
        false => management_mode::management_mode(&args),
    };
    crate::__wsllink_dbg!("Child WSL proc exitcode", exit_code); // debug msg

    // exit with exitcode
    std::process::exit(exit_code.unwrap_or(-1));
}

/// Get basename of original binary
fn orig_binname() -> Option<String> {
    std::env::current_exe() // Result<PathBuf> link_or_bin_fullpath
        .and_then(|pb| pb.canonicalize()) // .. -> Result<PathBuf> link_resolved_path
        .ok()? // .. -> PathBuf
        .path_basename() // .. -> Option<&str> basename
        .get_owned() // .. -> Option<String> basename
}

/// Get basename of binary from command line argument
fn cmdarg_binname(cmd_args: &[String]) -> Option<String> {
    cmd_args
        .get(0)? // &String fullpath
        .path_basename() // Option<&str> basename
        .get_owned() // Option<String>
}

/// Make console window hidden
fn windows_freeconsole() {
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != std::ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}
