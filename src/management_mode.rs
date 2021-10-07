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
                        .map_err(|e| {
                            cprintln!(Color::Red, " * Failed to link command '{}': {}", s_cmd, e);

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
        (Some("del"), Some(_)) | (Some("rm"), Some(_)) => {
            match args[2..]
                // do all jobs for each arg
                .iter()
                .map(|s_cmd| {
                    wslcmd_list
                        .unlink_wslcmd(s_cmd)
                        // if s_cmd is error
                        .map_err(|e| {
                            cprintln!(Color::Red, " * Failed to unlink command '{}': {}", s_cmd, e);

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

        // list
        (Some("li"), _) | (Some("list"), _) => {
            use itertools::Itertools;
            use std::io::Write;
            use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

            // colored output writer
            let buf_writer = BufferWriter::stdout(ColorChoice::Auto);
            let mut buf = buf_writer.buffer();

            // build and print wslcmd list string
            {
                // write header
                write!(&mut buf, " - WSL command list ({}) :  ", binname)
            }
            .and_then(|_| {
                {
                    // get iter of all sorted cmdlist
                    wslcmd_list.cmdlist().iter().sorted()
                }
                // do for all list
                .all(|pb| {
                    {
                        // get basename of current cmdlist
                        pb.wlpath_basename().ok_or(())
                    }
                    .and_then(|s| {
                        // write list to buf
                        write!(&mut buf, "'")
                            .and_then(|_| {
                                buf.set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                            })
                            .and_then(|_| write!(&mut buf, "{}", s))
                            .and_then(|_| buf.set_color(ColorSpec::new().set_reset(true)))
                            .and_then(|_| write!(&mut buf, "'  "))
                            .map_err(|_| ())
                    })
                    .is_ok()
                })
                .then(|| ())
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Color print error",
                ))
            })
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
