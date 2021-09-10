use super::wsllink_core::wslcmd::WslCmd;

pub fn execution_mode(args: &[String]) -> Option<i32> {
    println!("\n ===== Execution mode! ===== ");

    println!("WslCmd::new( &{:?} )", args);
    if let Some(wsl_cmd) = WslCmd::new(&args) {
        println!("wsl_cmd = {:?}", wsl_cmd);

        println!("WslCmd::execute()");
        wsl_cmd.execute()
    } else {
        println!("WslCmd::new failed!");
        None
    }
}
