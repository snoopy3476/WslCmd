use super::libwsllink::WLPath;
use super::libwsllink::WslCmdList;

/// Manage (add/del/list) linked WSL commands
pub fn management_mode(args: &[String]) -> Result<(), i32> {
    crate::__wsllink_dbg!("Management mode - cmdline args", &args); // debug

    let binname = args
        .get(0)
        .and_then(WLPath::wlpath_basename)
        .unwrap_or_default();

    let mut wslcmd_list =
        std::env::current_exe().map_or_else(|_| Err(1), |pb| WslCmdList::new(&pb).ok_or(1))?;
    crate::__wsllink_dbg!("Management mode - WslCmdList", &wslcmd_list); // debug

    // branch based on first arg
    match (args.get(1).map(String::as_str), args.get(2)) {
        // link
        (Some("new"), Some(_)) | (Some("add"), Some(_)) | (Some("ln"), Some(_)) => {
            match args[2..]
                // do all jobs for each arg
                .iter()
                .map(|s_cmd| {
                    wslcmd_list
                        .link_wslcmd(s_cmd)
                        // if s_cmd is error
                        .map_err(|e| println!(" * Failed to link command '{}': {}", s_cmd, e))
                        .is_ok()
                })
                // check if there is failed job
                // * calling 'all' directly without collecting
                //   leads to immediate stop right after fail
                .collect::<Vec<bool>>()
                .iter()
                .all(|is_ok| *is_ok)
            {
                true => {
                    println!(" - Linked all commands successfully");
                    Ok(()) // return ok
                }
                false => {
                    println!(" * Failed to link some commands while working!");
                    Err(-1) // return err
                }
            }
        }

        // unlink
        (Some("del"), Some(_)) | (Some("rm"), Some(_)) => {
            match args[2..]
                // do all jobs for each arg
                .iter()
                .map(|s_cmd| {
                    wslcmd_list
                        .unlink_wslcmd(s_cmd)
                        // if s_cmd is error
                        .map_err(|e| println!(" * Failed to unlink command '{}': {}", s_cmd, e))
                        .is_ok()
                })
                // check if there is failed job
                // * calling 'all' directly without collecting
                //   leads to immediate stop right after fail
                .collect::<Vec<bool>>()
                .iter()
                .all(|is_ok| *is_ok)
            {
                true => {
                    println!(" - Unlinked all commands successfully");
                    Ok(()) // return ok
                }
                false => {
                    println!(" * Failed to unlink some commands while working!");
                    Err(-1) // return err
                }
            }
        }

        // list
        (Some("li"), _) | (Some("list"), _) => {
            print!(" [WSL command list - {}] \n{}\n", binname, wslcmd_list);
            Ok(()) // return ok
        }

        // default
        _ => {
            print_help(binname);
            Err(-1) // return err
        }
    }
}

fn print_help(bin_name: &str) {
    print!(
        concat!(
            "usage: {0} <operation> [<arg1> <arg2> ...]\n",
            "\n",
            "  <operation>\n",
            "\n",
            "    - Link new commands\n",
            "\n",
            "        {0} new <command-name-1> <command-name-2> ...\n",
            "        {0} add <command-name-1> <command-name-2> ...\n",
            "        {0} ln <command-name-1> <command-name-2> ...\n",
            "\n",
            "    - Unlink existing commands\n",
            "\n",
            "        {0} del <command-name-1> <command-name-2> ...\n",
            "        {0} rm <command-name-1> <command-name-2> ...\n",
            "\n",
            "    - List linked commands\n",
            "\n",
            "        {0} li\n",
            "        {0} list\n",
            "\n"
        ),
        bin_name
    );
}
