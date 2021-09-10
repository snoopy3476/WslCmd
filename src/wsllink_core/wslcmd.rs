use derive_getters::Getters;

use super::common::file_basename;

/// delimiter of command name, which divides into bg proc mode, wsl command name, wsl user name
const CMDNAME_DELIM: char = '$';

#[derive(Getters, Debug)]
/// Store input WSL cmdline info including arguments,
/// which can be converted to execute WSL command
pub struct WslCmd {
    _no_direct_construct: (),

    /// WSL command name
    pub wsl_command: String,
    /// WSL command arguments
    pub wsl_args: Vec<String>,

    /// WSL username to execute command
    pub wsl_proc_user: Option<String>,
    /// Background process mode (GUI mode)
    pub wsl_proc_detached_mode: bool,
}

impl WslCmd {
    ///
    /// Create new [`WslCmd`]
    ///
    /// # Arguments
    ///
    /// * `cmd_args` - A full command-line arguments, including a command name (cmd_args\[0\])
    ///
    /// # Return
    ///
    /// A newly created [`WslCmd`] with given cmdline args
    ///
    /// # Examples
    ///
    /// ```
    /// let args: Vec<String> = args().collect();
    /// let wsl_cmd: WslCmd = WslCmd::new(&args);
    /// ```
    ///
    pub fn new(cmd_args: &[String]) -> Option<Self> {
        // split given args into (binname, args)
        cmd_args.split_first().map(|(binname, args)| {
            let (wsl_proc_detached_mode, wsl_command, wsl_proc_user) = Self::parse_cmd(binname);
            let wsl_args = Self::parse_args(args);

            // return struct instance
            Self {
                _no_direct_construct: (),
                wsl_proc_detached_mode,
                wsl_command,
                wsl_proc_user,
                wsl_args,
            }
        })
    }

    // parse command name, to get (detached mode, command, user)
    fn parse_cmd(binname: &String) -> (bool, String, Option<String>) {
        let mut binname_it = {
            file_basename(binname)
                .split(CMDNAME_DELIM) // iterator by splitted binname
                .peekable()
        };
        (
            binname_it.next_if(|str| str.is_empty()).is_some(), // detached mode
            binname_it.next().unwrap_or("").to_owned(),         // command
            binname_it.next().map(&str::to_owned),              // user
        )
    }

    // parse each arg and do processing
    fn parse_args(args: &[String]) -> Vec<String> {
        args.iter().map(|s| s.to_owned()).collect()
    }

    ///
    /// Execute [`WslCmd`].
    ///
    /// # Return
    ///
    /// [`Some`]\(exit_code\) if the command is executed, [`None`] if failed before execution.
    ///   (exit_code will be 0 when executed in [`bg_proc_mode`](Self::bg_proc_mode)
    ///
    /// # Examples
    ///
    /// ```
    /// let wsl_cmd: WslCmd = WslCmd::new(&args);
    /// let exit_code: Option<i32> = wsl_cmd.execute();
    /// ```
    ///
    pub fn execute(&self) -> Option<i32> {
        use std::os::windows::process::CommandExt;

        // build and execute command, then get exit code
        std::process::Command::new("wsl")
            // append arg: username
            .args({
                self.wsl_proc_user
                    .as_deref()
                    // if wsl_user_to_exec defined, add additional arguments
                    .map_or(
                        vec![],                  // user not defined
                        |user| vec!["-u", user], // user defined
                    )
            })
            // append arg: start wsl shell commands
            .arg("--")
            // append args: load env vars
            .args(&[".", "/etc/profile;", ".", "$HOME/.profile;"])
            // append arg: append wsl command
            .arg(&self.wsl_command)
            // append args: wsl command args
            .args(&self.wsl_args)
            // set flag: create as normal mode or GUI mode
            .creation_flags({
                if !self.wsl_proc_detached_mode {
                    0x00000000 // cli mode flag - RESET
                } else {
                    0x08000000 // gui mode flag - CREATE_NO_WINDOW
                }
            })
            // execute command in a bg child process
            .spawn() // Result<Child>
            // handle with process exit status
            .map_or(None, |mut child| {
                // wait and get exit code, if non-bg process
                if !self.wsl_proc_detached_mode {
                    child
                        .wait() // attach bg child process to fg
                        .map_or(None, |exitstatus| exitstatus.code()) // extract exit code
                } else {
                    Some(0) // for bg process mode
                }
            }) // Option<i32>
    }
}
