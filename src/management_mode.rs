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
    match (
        args.get(1).filter(|s| !s.is_empty()).map(String::as_str),
        args.get(2).filter(|s| !s.is_empty()),
    ) {
        // ops with more than 2 args
        (Some(op), Some(_)) => {
            // link
            if ["add", "new"].iter().any(|s| s.starts_with(op)) {
                match args[2..]
                    // do all jobs for each arg
                    .iter()
                    .map(|s_cmd| {
                        wslcmd_list
                            .link_wslcmd(s_cmd)
                            // if s_cmd is error
                            .map_err(|e| {
                                cprintln!(
                                    Color::Red,
                                    " * Failed to link command '{}': {}",
                                    s_cmd,
                                    e
                                );

                                e // bypass Err
                            })
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
                        cprintln!(Color::Green, " - Linked command(s) successfully");

                        Ok(()) // return ok
                    }
                    false => {
                        cprintln!(Color::Red, " * Failed to link some commands while working!");

                        Err(-1) // return err
                    }
                }
            }
            // unlink
            else if ["del", "rm"].iter().any(|s| s.starts_with(op)) {
                match args[2..]
                    // do all jobs for each arg
                    .iter()
                    .map(|s_cmd| {
                        wslcmd_list
                            .unlink_wslcmd(s_cmd)
                            // if s_cmd is error
                            .map_err(|e| {
                                cprintln!(
                                    Color::Red,
                                    " * Failed to unlink command '{}': {}",
                                    s_cmd,
                                    e
                                );

                                e // bypass Err
                            })
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
                        cprintln!(Color::Green, " - Unlinked command(s) successfully");

                        Ok(()) // return ok
                    }
                    false => {
                        cprintln!(
                            Color::Red,
                            " * Failed to unlink some commands while working!"
                        );

                        Err(-1) // return err
                    }
                }
            }
            // default
            else {
                print_help(binname);
                Err(-1) // return err
            }
        }

        // ops with 1 arg
        (Some(op), None) => {
            // link
            if ["list", "ls"].iter().any(|s| s.starts_with(op)) {
                use itertools::Itertools;
                use std::io::Write;
                use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

                // colored output writer
                let buf_writer = BufferWriter::stdout(ColorChoice::Auto);
                let mut buf = buf_writer.buffer();

                // build and print wslcmd list string
                {
                    // get iter of all sorted cmdlist
                    Some(wslcmd_list.get_cmdlist().iter().sorted())
                }
                .filter(|i| i.clone().count() > 0)
                .map_or_else(
                    // if no entry
                    || {
                        cprint!(Color::Yellow, "(No linked WSL command)");
                        true
                    },
                    // if WSL commands exist
                    |mut i| {
                        // do for all list
                        i.all(|pb| {
                            {
                                // get basename of current cmdlist
                                pb.wlpath_basename().ok_or(())
                            }
                            .and_then(|s| {
                                // if current string contains ws, wrap with '
                                {
                                    s.contains(char::is_whitespace)
                                        .then(|| ("'", "'")) // wrapper front/back
                                        .or_else(|| Some(("", ""))) // no wrapper
                                }
                                // print using 's' and 'wrap_*' data
                                .map_or(
                                    Ok(()),
                                    |(wrap_front, wrap_back)| {
                                        // write list to buf
                                        write!(&mut buf, "{}", wrap_front)
                                            .and_then(|_| {
                                                buf.set_color(
                                                    ColorSpec::new().set_fg(Some(Color::Green)),
                                                )
                                            })
                                            .and_then(|_| write!(&mut buf, "{}", s))
                                            .and_then(|_| {
                                                buf.set_color(ColorSpec::new().set_reset(true))
                                            })
                                            .and_then(|_| write!(&mut buf, "{}", wrap_back))
                                            .and_then(|_| write!(&mut buf, "\t"))
                                            .map_err(|_| ())
                                    },
                                )
                            })
                            .is_ok()
                        })
                    },
                )
                .then(|| ())
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Color print error",
                ))
                // end with newline
                .and_then(|_| writeln!(&mut buf))
                // reset color at the end
                .and_then(|_| buf.set_color(ColorSpec::new().set_reset(true)))
                // print built buf to terminal
                .and_then(|_| buf_writer.print(&buf))
                .and_then(|_| Ok(buf.clear()))
                .map_err(|_| -1)
            }
            // default
            else {
                print_help(binname);
                Err(-1) // return err
            }
        }

        // default
        _ => {
            print_help(binname);
            Err(-1) // return err
        }
    }
}

fn print_help(bin_name: &str) {
    let bin_name_blank = format!("{: ^1$}", " ", bin_name.len());
    print!(
        concat!(
            "usage: {0} <operation> [<arg1> <arg2> ...]\n",
            "\n",
            "  <operation>\n",
            "\n",
            "    - Link new commands\n",
            "\n",
            "        {0} add <command-name-1> (<command-name-2>) ...\n",
            "        {1} a          \"                 \"          ...\n",
            "        {1} new        \"                 \"          ...\n",
            "        {1} n          \"                 \"          ...\n",
            "\n",
            "    - Unlink existing commands\n",
            "\n",
            "        {0} del <command-name-1> (<command-name-2>) ...\n",
            "        {1} d          \"                 \"          ...\n",
            "        {1} rm         \"                 \"          ...\n",
            "        {1} r          \"                 \"          ...\n",
            "\n",
            "    - List linked commands\n",
            "\n",
            "        {0} list\n",
            "        {1} ls\n",
            "        {1} l\n",
            "\n"
        ),
        bin_name, bin_name_blank
    );
}
