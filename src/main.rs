#![crate_type = "bin"]
#![crate_name = "wslcmd"]
#![windows_subsystem = "console"]

/// Exporting macros for color print
#[macro_use]
mod color_print;

/// Exporting macros for debug print
#[macro_use]
mod dbg_print;

/// A module for executing WSL commands
mod execution_mode;
/// A module for managing WslCmd itself
mod management_mode;

/// Core routines of WslCmd
mod libwslcmd;
use libwslcmd::wcstr::*;

/// Branch routine (Management/Execution mode),
/// by checking if the binary is executed directly or through symlink
fn main() {
    // debug msg
    __wslcmd_dbg!("* Executed in debug mode! Debug msgs will be printed. *");

    // init
    let args: Vec<String> = std::env::args().collect();

    // call either mode, then get exitcode
    let exit_code = match is_exemode(args.get(0)) {
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
            cprintln!(Color::Red, "An unknown error occurred while checking args!");
            Err(1)
        }
    };
    __wslcmd_dbg!("Child WSL proc exitcode", &exit_code); // debug msg

    // exit with exitcode
    std::process::exit(exit_code.map_or_else(|code| code, |_| 0));
}

/// Check if execution mode,
/// by (orig_binname != cmdline_binname)
fn is_exemode<T: WCStr>(cmdname: T) -> Option<bool> {
    Some(
        // comparison
        {
            // current exe basename
            std::env::current_exe() // Result<PathBuf> link_or_bin_fullpath
                .ok()? // return None if failed
                .wcpath_canonicalize()? // resolve all links, return None if failed
                .wcpath_fstem() // slice basename
        } != {
            // command-line basename
            cmdname.wcpath_fstem() // slice basename
        },
    )
}
