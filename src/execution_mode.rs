use super::wsllink_core::wslcmd::WslCmd;

/// Convert Windows cmdline to WSL cmdline, then execute converted WSL command
pub fn execution_mode(args: &[String]) -> Option<i32> {
    crate::__wsllink_dbg!("Execution mode - cmdline args", args); // debug msg

    // if WslCmd is created
    if let Some(wsl_cmd) = WslCmd::new(&args) {
        crate::__wsllink_dbg!("Execution mode - 'WslCmd' created", &wsl_cmd); // debug msg
        wsl_cmd.execute() // return exitcode of WSL child proc
    }
    // if failed to create
    else {
        crate::__wsllink_dbg!("Execution mode - failed to create 'WslCmd'"); // debug msg
        None // return None
    }
}
