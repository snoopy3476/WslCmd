use super::libwsllink::WslCmd;

/// Convert Windows cmdline to WSL cmdline, then execute converted WSL command
pub fn execution_mode(args: &[String]) -> Result<(), i32> {
    __wsllink_dbg!("Execution mode - cmdline args", args); // debug msg

    // if WslCmd is created
    if let Some(wsl_cmd) = WslCmd::new(&args) {
        __wsllink_dbg!("Execution mode - 'WslCmd' created", &wsl_cmd); // debug msg
        wsl_cmd
            .execute()
            // return exitcode of WSL child proc
            .map_or_else(|r| Err(r.code.unwrap_or(-1)), |_| Ok(()))
    }
    // if failed to create
    else {
        __wsllink_dbg!("Execution mode - failed to create 'WslCmd'"); // debug msg
        Err(1) // return None
    }
}
