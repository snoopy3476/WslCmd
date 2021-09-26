use derive_getters::Getters;

use super::WLPath;
use super::WLStr;

/// Delimiter of command name, which divides into bg proc mode, wsl command name, wsl user name
const CMDNAME_DELIM: char = '$';
/// Name of env arg, which prevent argument path conversion if set
const ENVFLAG_NO_ARGCONV: &str = "WSLLINK_NO_ARGCONV";

#[derive(Getters, Debug)]
/// Store input WSL cmdline info including arguments,
/// which can be converted to execute WSL command
pub struct WslCmd {
    /// WSL command name
    command: String,

    /// WSL command arguments
    args: Vec<String>,

    /// WSL username to execute command
    username: Option<String>,

    /// WSL distribution
    distribution: Option<String>,

    /// Detached process mode
    ///
    /// Execute as a detached background process. Useful for GUI binaries.
    is_detached_proc: bool,
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
    /// A newly created [`Some`]\([`WslCmd`]\) with given cmdline args if succeeded.
    /// [`None`] if failed to create an instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let args: Vec<String> = args().collect();
    /// let wslcmd: Option<WslCmd> = WslCmd::new(&args);
    /// ```
    ///
    #[allow(dead_code)]
    pub fn new<T: WLStr>(cmd_args: &[T]) -> Option<Self> {
        // split given args into (binname, args)
        cmd_args.split_first().and_then(|(binname, args)| {
            // get vars from binname with parsing
            let (is_detached_proc, command, username, distribution) = Self::parse_cmd(binname)?;
            // parse & convert args to wsl args
            let args = Self::parse_args(args);

            // return struct instance
            Some({
                Self {
                    is_detached_proc,
                    command,
                    username,
                    distribution,
                    args,
                }
            })
        })
    }

    ///
    /// Execute [`WslCmd`].
    ///
    /// # Return
    ///
    /// [`Some`]\(exit_code\) if the command is executed, [`None`] if failed before execution.
    ///   (exit_code will be 0 when executed in [`is_detached_proc`](Self::is_detached_proc)
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd: Option<WslCmd> = WslCmd::new(&args);
    /// let exit_code: Option<i32> = wslcmd.map_or(None, |w| w.execute());;
    /// ```
    ///
    #[allow(dead_code)]
    pub fn execute(&self) -> Option<i32> {
        use std::os::windows::process::CommandExt;

        // build and execute command, then get exit code
        std::process::Command::new("wsl")
            // append arg: username
            .args({
                match self.username.as_deref() {
                    Some(user) => vec!["-u", user], // user defined: additional args
                    None => vec![],                 // user not defined: no args
                }
            })
            // append arg: distribution
            .args({
                match self.distribution.as_deref() {
                    Some(dist) => vec!["-d", dist], // user defined: additional args
                    None => vec![],                 // user not defined: no args
                }
            })
            // append arg: start wsl shell commands
            .arg("--")
            // append args: load env vars
            .args(&[".", "/etc/profile;", ".", "$HOME/.profile;"])
            // append arg: append wsl command
            .arg(&self.command)
            // append args: wsl command args
            .args(&self.args)
            // set flag: create as normal mode or detached mode
            .creation_flags({
                match self.is_detached_proc {
                    // https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
                    true => 0x08000000,  // detached mode flag - CREATE_NO_WINDOW
                    false => 0x00000000, // normal mode flag - RESET
                }
            })
            // execute command in a bg child process first (attached later if needed)
            .spawn() // Result<Child>
            .ok()
            // handle with process exit status
            .and_then(|mut child| {
                match self.is_detached_proc {
                    true => Some(0),                    // for bg process mode
                    false => child.wait().ok()?.code(), // extract exit code
                }
            })
    }

    // parse command name, to get (detached mode, command, user)
    // returns None if error (failed to get basename, command name is empty)
    fn parse_cmd<T: WLPath>(binname: &T) -> Option<(bool, String, Option<String>, Option<String>)> {
        let mut it = {
            binname
                .wlpath_basename()?
                .split(CMDNAME_DELIM) // iterator by splitted binname
                .peekable()
        };
        Some((
            it.next_if(|str| str.is_empty()).is_some(), // detached mode
            it.next().map(String::from).filter(|s| !s.is_empty())?, // command, must not empty
            it.next().map(String::from).filter(|s| !s.is_empty()), // user
            it.next().map(String::from).filter(|s| !s.is_empty()), // distribution
        ))
    }

    // parse each arg and do processing
    fn parse_args<T: WLStr>(args: &[T]) -> Vec<String> {
        match std::env::var(ENVFLAG_NO_ARGCONV).is_err() {
            // default: convert args
            true => args.iter().map(Self::convert_arg_to_wsl_arg).collect(),
            // if NO_ARGCONV flag set, no conversion
            false => args
                .iter()
                .map(|t| t.wlstr_as_ref().unwrap_or_default().to_owned())
                .collect(),
        }
    }

    // arg -> wsl arg (mainly path conversion)
    fn convert_arg_to_wsl_arg<T: WLStr>(arg: &T) -> String {
        arg.wlstr_invoke(Self::arg_convert_and_unescape_backslashes)
            .wlstr_invoke(Self::arg_wrap_with_wslpath_if_absolute)
            .unwrap_or(arg.wlstr_as_ref().unwrap_or_default().to_owned())
    }

    // replace single '\' (not consecutive '\'s) to '/',
    // then remove one '\' from consecutive '\'s
    //   Ex) '\' -> '/'
    //        '\\' -> '\'
    //        '\\\' -> '\\'
    //        ...
    fn arg_convert_and_unescape_backslashes<T: WLStr>(arg: &T) -> Option<String> {
        arg.wlstr_replace_all_regex(
            concat!(r"(?P<pre>(^|[^\\]))", r"\\", r"(?P<post>([^\\]|$))"),
            "$pre/$post",
        ) // '\' -> '/'
        .wlstr_replace_all_regex(r"\\(?P<remain>\\+)", "$remain") // '\\...' -> '\...'
    }

    // if an argument an absolute path, just converting '\' -> '/' is not enough.
    // the arg starting with drive letter pattern should be converted into wsl path manually
    // using wslpath inside wsl.
    fn arg_wrap_with_wslpath_if_absolute<T: WLStr>(arg: &T) -> Option<String> {
        arg.wlstr_as_ref()
            .wlpath_is_absolute()
            // wrap with wslpath substitution
            .then(|| format!("$(wslpath '{}')", arg.wlstr_as_ref().unwrap_or_default()))
    }
}
