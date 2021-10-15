// crate
use super::libwslcmd::wcstr::*;
use super::libwslcmd::WslCmd;

/// Name of env arg, which prevent argument path conversion if set
const ENVFLAG_NO_ARGCONV: &str = "WSLCMD_NO_ARGCONV";

/// Convert Windows cmdline to WSL cmdline, then execute converted WSL command
pub fn execution_mode(args: &[String]) -> Result<(), i32> {
    __wslcmd_dbg!("Execution mode - cmdline args", args); // debug msg

    // environment files to load with WSL shell before command execute
    let custom_envfiles = &[
        // 'profile' file inside the current exe dir
        std::env::current_exe()
            .ok()
            .and_then(|pb| {
                // convert '\' (win-path dir delim) to '/',
                // then and wrap with wslpath as cur_exe is win-path
                pb.with_file_name("profile").to_str().and_then(|s| {
                    Some(format!(
                        "$(wslpath '{}')",
                        // escape ' inside quote-str
                        s.replace("'", r"'\''")
                    ))
                })
            })
            .unwrap_or_default(),
    ];

    // execute, and return process exitcode
    args.split_first() // split into cmd + args
        .and_then(|(cmd, args)| parse_cmd(cmd).map(|t| (t, args)))
        .ok_or(-1) // Option -> Result
        .and_then(|((cmd, user, dist), args)| {
            // build wslcmd
            {
                WslCmd::new(cmd)
                    .map(|w| {
                        w
                            // set args
                            .args(args, std::env::var(ENVFLAG_NO_ARGCONV).is_err())
                            // set user
                            .user(user)
                            // set distribution
                            .dist(dist)
                            // set env files
                            .envfiles(custom_envfiles)
                    })
                    .ok_or(-1)?
            }
            // execute wslcmd & map error
            .execute()
            .map_or_else(|res| Err(res.code.unwrap_or(-1)), |_| Ok(()))
        })
}

/// Delimiter of command name, which divides into bg proc mode, wsl command name, wsl user name
const CMDNAME_DELIM: char = '!';

// parse command name, to get (detached mode, command, user)
// returns None if error (failed to get basename, command name is empty)
fn parse_cmd<T: WCStr>(binname: T) -> Option<(String, Option<String>, Option<String>)> {
    binname
        .wcpath_fstem()
        .map(|cmd| {
            cmd.split(CMDNAME_DELIM) // iterator by splitted binname
                .peekable()
        })
        .and_then(|mut it| {
            Some((
                it.next().map(String::from).filter(|s| !s.is_empty())?, // command, must not empty
                it.next().map(String::from).filter(|s| !s.is_empty()),  // user
                it.next().map(String::from).filter(|s| !s.is_empty()),  // distribution
            ))
        })
}
