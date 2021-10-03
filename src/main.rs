#![crate_type = "bin"]
#![crate_name = "wsllink"]

/// A module for executing WSL commands
mod execution_mode;
/// A module for managing WslLink itself
mod management_mode;

/// Core routines of WslLink
mod libwsllink;
use libwsllink::WLPath;
use libwsllink::WLStr;

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
    let exit_code = match is_exemode(&args) {
        // comparison succeeded
        Some(ret) => {
            match ret {
                // if executed through command symlink
                true => execution_mode::execution_mode(&args),

                // if executed directly
                // (including symlinks of which basename == 'orig bin basename')
                false => management_mode::management_mode(&args),
            }
        }

        // comparison failed for unknown reason
        _ => {
            eprintln!("An unknown error occurred while checking args!");
            Err(1)
        }
    };
    crate::__wsllink_dbg!("Child WSL proc exitcode", exit_code).ok(); // debug msg

    // exit with exitcode
    std::process::exit(exit_code.map_or_else(|code| code, |_| 0));
}

/// Check if execution mode,
/// by (orig_binname != cmdline_binname)
fn is_exemode<T: WLStr>(cmd_args: &[T]) -> Option<bool> {
    Some(
        // comparison
        {
            // current exe basename
            std::env::current_exe() // Result<PathBuf> link_or_bin_fullpath
                .ok()? // return None if failed
                .wlpath_canonicalize()? // resolve all links, return None if failed
                .wlpath_basename() // slice basename
        } != {
            // command-line basename
            cmd_args
                .get(0)? // fullpath, return None if failed
                .wlpath_basename() // slice basename
        },
    )
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
