use derive_getters::Getters;

use super::WCPath;
use super::WCStr;

use super::DETACHED_PROC_PREFIX;

#[derive(Getters, Debug)]
/// Store input WSL cmdline info including arguments,
/// which can be converted to execute WSL command
pub struct WslCmd {
    /// WSL command name
    #[getter(rename = "get_command")]
    command: String,

    /// WSL command arguments
    #[getter(rename = "get_args")]
    args: Vec<String>,

    /// WSL username to execute command
    #[getter(rename = "get_user")]
    username: Option<String>,

    /// WSL distribution
    #[getter(rename = "get_dist")]
    distribution: Option<String>,

    /// WSL envfile list
    #[getter(rename = "get_envfiles")]
    envfiles: Vec<String>,

    /// Detached process mode
    ///
    /// Execute as a detached background process. Useful for GUI binaries.
    #[getter(rename = "get_is_detached")]
    is_detached_proc: bool,
}

impl WslCmd {
    ///
    /// Create new [`WslCmd`]
    ///
    /// # Arguments
    ///
    /// * `cmdname` - Name of command
    ///
    /// # Return
    ///
    /// A newly created [`Some`]\([`WslCmd`]\) with given command name if succeeded.
    /// [`None`] if failed to create an instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd: Option<WslCmd> = WslCmd::new("command");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn new<T: WCStr>(cmdname: T) -> Option<Self> {
        // parse cmd, return None if failed at this point
        let (command, is_detached_proc) = Self::parse_cmd(&cmdname)?;

        // return struct instance
        Some({
            Self {
                command,
                is_detached_proc,
                args: [].to_vec(),     // default
                username: None,        // default
                distribution: None,    // default
                envfiles: [].to_vec(), // default
            }
        })
    }

    ///
    /// Set arguments for [`WslCmd`]
    ///
    /// # Arguments
    ///
    /// * `args`             - Arguments list
    /// * `convert_pathargs` - If set, convert Windows path arguments to WSL path
    ///
    /// # Return
    ///
    /// Self [`WslCmd`] after setting arguments
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd: WslCmd = WslCmd::new("ls")
    ///            .expect("New WslCmd")
    ///            .args(&["C:/Users", "D:/", "relpath-dir"], true);
    /// ```
    ///
    #[allow(dead_code)]
    pub fn args<T: WCStr>(mut self, args: &[T], convert_pathargs: bool) -> Self {
        self.args = Self::parse_args(args, convert_pathargs);

        self
    }

    ///
    /// Set WSL user to execute [`WslCmd`]
    ///
    /// # Arguments
    ///
    /// * `username` - User name of WSL for the process
    ///
    /// # Return
    ///
    /// Self [`WslCmd`] after setting username
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd: WslCmd = WslCmd::new("command")
    ///            .expect("New WslCmd")
    ///            .user("ubuntuuser");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn user<T: WCStr>(mut self, username: T) -> Self {
        self.username = username.wcstr_clone_to_string();

        self
    }

    ///
    /// Set WSL distribution to execute [`WslCmd`]
    ///
    /// # Arguments
    ///
    /// * `distribution` - WSL Distribution name for the process
    ///
    /// # Return
    ///
    /// Self [`WslCmd`] after setting distribution
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd: WslCmd = WslCmd::new("command")
    ///            .expect("New WslCmd")
    ///            .dist("ubuntu");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn dist<T: WCStr>(mut self, distribution: T) -> Self {
        self.distribution = distribution.wcstr_clone_to_string();

        self
    }

    ///
    /// Set environment file to source for [`WslCmd`]
    ///
    /// # Arguments
    ///
    /// * `file_list` - File list to load before WSL execute
    ///
    /// # Return
    ///
    /// Self [`WslCmd`] after setting envfiles
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd: WslCmd = WslCmd::new("command")
    ///            .expect("New WslCmd")
    ///            .envfiles(&["path/to/file1", "path/to/file2"]);
    /// ```
    ///
    #[allow(dead_code)]
    pub fn envfiles<T: WCStr>(mut self, file_list: &[T]) -> Self {
        self.envfiles = file_list
            .iter()
            .map(|t| t.wcstr_clone_to_string().unwrap_or_default())
            .collect();

        self
    }

    ///
    /// Execute [`WslCmd`].
    ///
    /// # Return
    ///
    /// [`Ok`]\([`WslCmdExitStatus`]\) if the command is succeeded, [`Err`]\([`WslCmdExitStatus`]\) if failed.
    /// * For non-detached WSL command, [`Ok`] when the WSL command executed and returned exit code 0.
    /// * For detached WSL command \([`WslCmd`] with [`is_detached_proc`](Self::is_detached_proc) set\), [`Ok`] only means WSL command was executed, and does not necessarily mean exit code is 0.
    /// * If [`Err`] but the exit code is [`Some`]\(0\), there was an error before executing the WSL command.
    /// * If [`Err`] and the exit code is [`Some`]\([`i32`]\) \(Other than 0\), containing WSL command was executed and exited with the exit code.
    /// * If [`Err`] and the exit code is [`None`], containing WSL command was executed but terminated by a signal.
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd: WslCmd = WslCmd::new(&args).expect("New Wslcmd");
    /// let exit_status: WslCmdResult = wslcmd.execute();
    /// ```
    ///
    #[allow(dead_code)]
    pub fn execute(&self) -> WslCmdResult {
        self.execute_with_stdin(None)
    }

    ///
    /// Execute [`WslCmd`] with specific stdin input.
    ///
    /// # Arguments
    ///
    /// * `stdin_input` - A string input, which is written to stdin of child. If this is [`Some`], then terminal output will not be printed, but stored inside return value instead.
    ///
    /// # Return
    ///
    /// Same as [`execute()`](Self::execute), but contains stdout and stderr output in returned [`WslCmdExitStatus`] if any.
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd: WslCmd = WslCmd::new(&["cat"]).expect("New Wslcmd");
    /// let exit_status: WslCmdResult = wslcmd.execute_with_stdin(Some("Stdin str"));
    /// let stdout_str: Option<String> = exit_status
    ///     .as_ref()
    ///     .ok()
    ///     .and_then(|e| e.stdout.as_ref())
    ///     .and_then(|s| String::from_utf8(s.to_vec()).ok());
    /// println!("{:?}, {:?}", exit_status, stdout_str);
    /// ```
    ///
    #[allow(dead_code)]
    pub fn execute_with_stdin(&self, stdin_input: Option<&str>) -> WslCmdResult {
        use std::io::Write;
        use std::os::windows::process::CommandExt;
        use std::process::{Command, Stdio};

        // build and execute command, then get exit code
        Command::new("wsl")
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
            .args(Self::buildcmd_load_envfile_if_exists(
                // load '/etc/profile', '$HOME/.profile', and files in 'self.envfiles'
                ["/etc/profile", "$HOME/.profile"]
                    .iter()
                    .map(|s| *s)
                    .chain(self.envfiles.iter().map(|s| s.as_str())),
            ))
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
            // set stdio - if stdin_input exists, set all to piped
            .stdin(match stdin_input {
                Some(_) => Stdio::piped(),
                None => Stdio::inherit(),
            })
            .stdout(match stdin_input {
                Some(_) => Stdio::piped(),
                None => Stdio::inherit(),
            })
            .stderr(match stdin_input {
                Some(_) => Stdio::piped(),
                None => Stdio::inherit(),
            })
            // execute command as a bg child process first (attached later if needed)
            .spawn() // Result<Child>
            // handle with process exit status
            .map_or(WslCmdExitStatus::err(), |mut child| {
                match self.is_detached_proc {
                    // for bg process mode
                    true => WslCmdExitStatus::ok(),

                    // extract exit code from child
                    false => {
                        // write stdin to child, if input exists
                        if let Some(stdin_input_str) = stdin_input {
                            child
                                .stdin
                                .take()
                                // write to stdin if available
                                .map_or(None, |mut child_stdin| {
                                    child_stdin.write_all(stdin_input_str.as_bytes()).ok()
                                })
                                // if error during previous jobs, return with the error
                                .ok_or(WslCmdExitStatus::err().unwrap_err())?;
                        };

                        // wait and extract results
                        child
                            .wait_with_output()
                            .map_or(WslCmdExitStatus::err(), |o| WslCmdExitStatus::new(o))
                    }
                }
            })
    }

    // parse command name, to get (detached mode, command)
    // returns None if error (failed to get basename, command name is empty, ...)
    fn parse_cmd<T: WCPath>(binname: &T) -> Option<(String, bool)> {
        binname
            // get basename
            .wcpath_basename()
            // basename to (cmd, detached)
            .and_then(
                // test if starts with DETACHED_PROC_PREFIX
                |basename| match basename.chars().next()? == DETACHED_PROC_PREFIX {
                    // detached proc: remove prefix from the basename -> cmd
                    true => basename.get(1..).map(|cmd| (cmd, true)),
                    // normal proc
                    false => Some((basename, false)),
                },
            )
            // None if cmd is empty
            .filter(|(cmd, _)| !cmd.is_empty())
            // cmd str to string
            .and_then(|(cmd, detached)| Some((cmd.wcstr_clone_to_string()?, detached)))
    }

    // parse each arg and do processing
    fn parse_args<T: WCStr>(args: &[T], convert: bool) -> Vec<String> {
        match convert {
            // convert args
            true => args.iter().map(Self::convert_arg_to_wsl_arg).collect(),
            // no conversion
            false => args
                .iter()
                .map(|t| t.wcstr_clone_to_string().unwrap_or_default())
                .collect(),
        }
    }

    // arg -> wsl arg (mainly path conversion)
    fn convert_arg_to_wsl_arg<T: WCStr>(arg: &T) -> String {
        arg.wcstr_invoke(Self::arg_convert_and_unescape_backslashes)
            // if arg_wrap_with_.. returns None, use input as output
            .wcstr_invoke(|s| Self::arg_wslpath_wrap_if_abs(s).or(s.wcstr_clone_to_string()))
            .unwrap_or_default()
    }

    // replace single '\' (not consecutive '\'s) to '/',
    // then remove one '\' from consecutive '\'s
    //   Ex) '\' -> '/'
    //        '\\' -> '\'
    //        '\\\' -> '\\'
    //        ...
    fn arg_convert_and_unescape_backslashes<T: WCStr>(arg: &T) -> Option<String> {
        arg.wcstr_replace_all_regex(
            concat!(r"(?P<pre>(^|[^\\]))", r"\\", r"(?P<post>([^\\]|$))"),
            "$pre/$post",
        ) // '\' -> '/'
        .wcstr_replace_all_regex(r"\\(?P<remain>\\+)", "$remain") // '\\...' -> '\...'
    }

    // if an argument an absolute path, just converting '\' -> '/' is not enough.
    // the arg starting with drive letter pattern should be converted into wsl path manually
    // using wslpath inside wsl.
    fn arg_wslpath_wrap_if_abs<T: WCStr>(arg: &T) -> Option<String> {
        arg.wcstr_as_ref()
            .filter(|s| s.wcpath_is_absolute())
            // escape ' inside quote-str, then wrap with wslpath substitution
            .map(|s| format!("$(wslpath '{}')", s.replace("'", r"'\''")))
    }

    // get env load string from file path
    fn buildcmd_load_envfile_if_exists<'a, I: Iterator<Item = &'a str>>(
        envfile_iter: I,
    ) -> Vec<&'a str> {
        envfile_iter
            .map(|s| ["if", "test", "-r", s, ";", "then", ".", s, ";", "fi;"].to_vec())
            .collect::<Vec<Vec<&str>>>()
            .concat() // flatten
    }
}

type WslCmdResult = Result<WslCmdExitStatus, WslCmdExitStatus>;

/// Exit status and output of executed WSL cmdline
#[derive(Debug)]
pub struct WslCmdExitStatus {
    /// Exit code
    pub code: Option<i32>,

    /// Printed stdout
    pub stdout: Option<Vec<u8>>,

    /// Printed stderr
    pub stderr: Option<Vec<u8>>,
}

impl WslCmdExitStatus {
    ///
    /// Create new [`WslCmdExitStatus`]
    ///
    /// # Arguments
    ///
    /// * `output` - Output return of child execution
    ///
    /// # Return
    ///
    /// [`Ok`]\([`WslCmdExitStatus`]\) if output status is success,
    /// [`Err`]\([`WslCmdExitStatus`]\) if not
    ///
    pub fn new(output: std::process::Output) -> WslCmdResult {
        let ret = Self {
            code: output.status.code(),
            stdout: Self::vec_wrap(output.stdout),
            stderr: Self::vec_wrap(output.stderr),
        };

        match output.status.success() {
            true => Ok(ret),
            false => Err(ret),
        }
    }

    /// Default [`Ok`] Result for [`WslCmdExitStatus`]
    #[inline]
    pub fn ok() -> WslCmdResult {
        Ok(Self {
            code: Some(0),
            stdout: None,
            stderr: None,
        })
    }

    /// Default [`Err`] Result for [`WslCmdExitStatus`]
    #[inline]
    pub fn err() -> WslCmdResult {
        Err(Self {
            code: Some(0),
            stdout: None,
            stderr: None,
        })
    }

    // wrap vector with Option
    // if vec len is 0, then return None
    fn vec_wrap<T>(vec: Vec<T>) -> Option<Vec<T>> {
        match vec.len() {
            0 => None,
            _ => Some(vec),
        }
    }
}

#[cfg(test)]
/// For module test
mod test {
    use super::{WslCmd, WslCmdExitStatus, DETACHED_PROC_PREFIX};

    #[test]
    fn test_execute_true() {
        // create WslCmd & run test
        WslCmd::new("true")
            .expect("New WslCmd")
            .execute()
            .expect("Execute WslCmd - true");
    }

    #[test]
    fn test_execute_false() {
        // create WslCmd & run test
        WslCmd::new("false")
            .expect("New WslCmd")
            .execute()
            .expect_err("Execute WslCmd - false");
    }

    #[test]
    fn test_execute_false_detached() {
        // create WslCmd & run test
        WslCmd::new(format!("{}false", DETACHED_PROC_PREFIX))
            .expect("New WslCmd")
            .execute()
            .expect("Execute WslCmd - false (detached)");
    }

    #[test]
    fn test_execute_wslpath() {
        // create WslCmd & run test
        WslCmd::new("command")
            .expect("New WslCmd")
            .args(&["-v", "wslpath"], false)
            .execute_with_stdin(Some("")) // no child output while testing
            // only for debug: bypass exit_status, with printing outputs
            .map_or_else(
                // if Err
                |e| {
                    print_stdout_stderr(&e);
                    Err(e)
                },
                // if Ok
                |e| {
                    print_stdout_stderr(&e);
                    Ok(e)
                },
            )
            .expect("Execute WslCmd - wslpath abspath");
    }

    #[test]
    fn test_execute_cat_with_stdin() {
        const INPUT: &str = "With cat, stdin and stdout should be the same";

        // create WslCmd & run test
        WslCmd::new("cat")
            .expect("New WslCmd")
            .execute_with_stdin(Some(INPUT)) // set input, and no child output
            // only for debug: bypass exit_status, with printing outputs
            .map_or_else(
                // if Err
                |e| {
                    print_stdout_stderr(&e);
                    Err(e)
                },
                // if Ok
                |e| {
                    print_stdout_stderr(&e);
                    Ok(e)
                },
            )
            .expect("Execute WslCmd - cat")
            .stdout
            .filter(|s| String::from_utf8_lossy(&s) == INPUT) // check stdout == stdin
            .expect("Validate WslCmd - cat");
    }

    // print
    fn print_stdout_stderr(e: &WslCmdExitStatus) {
        e.stdout
            .as_ref()
            .map(|stdout| dbg!(String::from_utf8_lossy(stdout)));
        e.stderr
            .as_ref()
            .map(|stderr| dbg!(String::from_utf8_lossy(stderr)));
    }
}
