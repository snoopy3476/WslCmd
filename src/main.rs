#![crate_type = "bin"]
#![crate_name = "wsllink"]
//#![windows_subsystem = "windows"]

use std::env::{args, current_exe};

mod execution_mode;
mod management_mode;
mod wsllink_core;
use wsllink_core::common::file_basename;

fn main() {
    windows_freeconsole();
    let args: Vec<String> = args().collect();

    // get exit code from either mode
    let exit_code = {
        // if executed through command symlink
        if orig_binname() != cmdarg_binname(&args) {
            execution_mode::execution_mode(&args)
        }
        // if executed directly
        // (including symlinks of which basename == 'orig bin basename')
        else {
            management_mode::management_mode(&args)
        }
    };

    println!(
        "exit_code: {}",
        exit_code.map_or("(None)".to_owned(), |code| code.to_string())
    );

    std::process::exit(exit_code.unwrap_or(-1));
}

fn orig_binname() -> Option<String> {
    current_exe() // Result<PathBuf> link_or_bin_fullpath
        .map_or(None, |pb| pb.canonicalize().ok()) // Option<PathBuf> link_resolved_fullpath
        .map_or(None, |pb| {
            Some(String::from(file_basename(pb.to_str().unwrap_or(""))))
        }) // .. -> Option<String> bin_basename
}

fn cmdarg_binname(cmd_args: &[String]) -> Option<String> {
    cmd_args
        .get(0) // Option<String> fullpath
        .as_deref() // Option<&str> fullpath
        .map(|str| file_basename(str).to_owned()) // Option<String> basename
}

/// Make console windows hidden
fn windows_freeconsole() {
    //unsafe { winapi::um::wincon::FreeConsole() };

    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}
