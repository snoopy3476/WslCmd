use super::libwsllink::WLPath;
use super::libwsllink::WslCmdList;

/// Manage (add/del/list) linked WSL commands
pub fn management_mode(args: &[String]) -> Result<(), i32> {
    crate::__wsllink_dbg!("Management mode - cmdline args", &args); // debug

    let mut wslcmd_list =
        std::env::current_exe().map_or_else(|_| Err(1), |pb| WslCmdList::new(&pb).ok_or(1))?;

    println!(" - WslCmdList - Before: [{}]", &wslcmd_list); // debug
    wslcmd_list.push("testcmd"); // test
    wslcmd_list.push("testcmd2"); // test
    wslcmd_list.push("testcmd3"); // test
    wslcmd_list.push("testcmd"); // test
    println!(" - WslCmdList - After:  [{}]", &wslcmd_list); // debug

    print_help(args.get(0).wlpath_basename().unwrap_or_default());

    Ok(())
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
            "        {0} list\n",
            "\n"
        ),
        bin_name
    );
}
