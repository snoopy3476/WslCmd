use super::libwsllink::{WLPath, WslCmd};

/// Name of env arg, which prevent argument path conversion if set
const ENVFLAG_NO_ARGCONV: &str = "WSLLINK_NO_ARGCONV";

/// Convert Windows cmdline to WSL cmdline, then execute converted WSL command
pub fn execution_mode(args: &[String]) -> Result<(), i32> {
    __wsllink_dbg!("Execution mode - cmdline args", args); // debug msg
    args.split_first()
        .and_then(|(cmd, args)| parse_cmd(cmd).map(|t| (t, args)))
        .ok_or(-1) // Option -> Result
        .and_then(|((cmd, user, dist), args)| {
            // build wslcmd
            {
                WslCmd::new(cmd)
                    .map(|w| {
                        w.args(args, std::env::var(ENVFLAG_NO_ARGCONV).is_err())
                            .user(user)
                            .dist(dist)
                    })
                    .ok_or(-1)?
            }
            // execute wslcmd & map error
            .execute()
            .map_or_else(|r| Err(r.code.unwrap_or(-1)), |_| Ok(()))
        })
}

/// Delimiter of command name, which divides into bg proc mode, wsl command name, wsl user name
const CMDNAME_DELIM: char = '!';

// parse command name, to get (detached mode, command, user)
// returns None if error (failed to get basename, command name is empty)
fn parse_cmd(binname: &String) -> Option<(String, Option<String>, Option<String>)> {
    let mut it = {
        binname
            .wlpath_basename()?
            .split(CMDNAME_DELIM) // iterator by splitted binname
            .peekable()
    };
    Some((
        it.next().map(String::from).filter(|s| !s.is_empty())?, // command, must not empty
        it.next().map(String::from).filter(|s| !s.is_empty()),  // user
        it.next().map(String::from).filter(|s| !s.is_empty()),  // distribution
    ))
}
