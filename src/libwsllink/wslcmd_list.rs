use derive_getters::Getters;

use super::{WLPath, WLStr};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::io;
use std::io::{Error, ErrorKind};
use std::ops::*;
use std::path::PathBuf;
use std::time::SystemTime;

use super::DETACHED_PROC_PREFIX;

/// Extension of Windows binary
const BINARY_EXTENSION: &str = "exe";

macro_rules! wslcmd_with_ext {
    ($label:expr) => {
        format!("{}.{}", $label, BINARY_EXTENSION)
    };
}

macro_rules! wslcmd_detached_bin {
    ($label:expr) => {
        format!("{}{}", DETACHED_PROC_PREFIX, $label)
    };
}

#[derive(Getters, Debug)]
/// Read dir and load all wslcmds, and manage the list
pub struct WslCmdList {
    /// Path of target bin
    #[getter(rename = "get_binpath")]
    binpath: PathBuf,

    /// Path of original bin, after following all symlinks
    #[getter(rename = "get_orig_binpath")]
    orig_binpath: PathBuf,

    /// Latest WslCmd list
    #[getter(rename = "get_cmdlist")]
    cmdlist: HashSet<String>,

    /// Time of WslCmd list data
    #[getter(rename = "get_cmdlist_time")]
    cmdlist_time: Option<SystemTime>,
}

impl WslCmdList {
    ///
    /// Create new [`WslCmdList`]
    ///
    /// # Arguments
    ///
    /// * `binpath` - A target wsllink bin path to get list of wslcmd
    ///
    /// # Return
    ///
    /// A newly created [`Some`]\([`WslCmdList`]\), initialized with wslcmd list.
    ///
    /// [`None`] if failed to initialize [`WslCmdList`].
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd_list = WslCmdList::new(&"/path/to/target/exe");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn new<T: WLPath>(binpath: &T) -> Option<Self> {
        // initialize basic info
        let binpath = binpath.wlpath_clone_to_pathbuf()?;
        let orig_binpath = binpath.wlpath_canonicalize()?;

        // build struct instance
        let mut ret_self = Self {
            binpath,
            orig_binpath,
            cmdlist: HashSet::new(), // dummy
            cmdlist_time: None,      // dummy
        };
        ret_self.refresh_wslcmd_list(); // refresh cmdlist and time

        Some(ret_self)
    }

    ///
    /// Link a new WSL command to current binary
    ///
    /// # Arguments
    ///
    /// * `cmdname` - A target command basename to link with WSL shell
    ///
    /// # Return
    ///
    /// [`Ok`]\([`()`](unit)\) if succeeded, [`Err`]\([`Error`]\) if failed
    ///
    /// # Examples
    ///
    /// ```
    /// let result = wslcmd_list.link_wslcmd("emacs");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn link_wslcmd<T: WLPath>(&mut self, cmdname: &T) -> io::Result<()> {
        // create new PathBuf of cmd: replace only filename with cmdname from binpath
        {
            cmdname
                .wlpath_filename() // get filename only, discarding possible parent dir name
                .and_then(|s_cmd| Some(wslcmd_with_ext!(s_cmd))) // append extension to cmdname
                .and_then(|s_file| Some(self.binpath.with_file_name(s_file))) // to abs path
                .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid cmdname"))
        }
        // Ok if valid cmdname
        .and_then(|pb_cmd| {
            self.orig_binpath
                .wlpath_basename()
                .and_then(|s_orig| pb_cmd.wlpath_filename().map(|s_cmd| (s_orig, s_cmd)))
                // bool expression
                .map_or(false, |(s_orig, s_cmd)| {
                    {
                        // cmdname is not the same with orig binname
                        s_orig != s_cmd
                    }
                    .bitand({
                        // cmdname is not detached cmd name pattern (starts with cmdname_delim
                        !s_cmd.starts_with(DETACHED_PROC_PREFIX)
                    })
                })
                // bool -> Result
                .then(|| pb_cmd)
                .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid cmdname"))
        })
        // Ok if given cmd is not wslcmd file
        .and_then(|pb_cmd| {
            // bool expression
            (!self.is_wslcmd_file(&pb_cmd))
                // bool -> Result
                .then(|| pb_cmd)
                .ok_or(Error::new(
                    ErrorKind::AlreadyExists,
                    "WslCmd already exists for given cmdname",
                ))
        })
        // create new symlink chain (wslcmd -> wslcmd_detached -> origbin)
        .and_then(|pb_cmd| {
            let wslcmd_detached_filename = wslcmd_detached_bin!(
                // wslcmd filename
                pb_cmd
                    .wlpath_filename()
                    .ok_or(Error::new(ErrorKind::Other, "Invalid cmdname"))?
            );

            // first create symlink (wslcmd_detached -> origbin)
            std::os::windows::fs::symlink_file(
                // target: origbin filename (relative)
                self.binpath
                    .wlpath_filename()
                    .ok_or(Error::new(ErrorKind::Other, "Invalid exe name"))?,
                // symlink file: wslcmd_detached (absolute)
                &pb_cmd.with_file_name(&wslcmd_detached_filename),
            )
            // if succeeded, create another symlink (wslcmd -> wslcmd_detached)
            .and_then(|()| {
                std::os::windows::fs::symlink_file(
                    // target: wslcmd_detached (relative)
                    &wslcmd_detached_filename,
                    // symlink file: wslcmd (absolute)
                    &pb_cmd,
                )
                // if second failed, clean progress (remove first created link)
                .or_else(|e| {
                    std::fs::remove_file(
                        // remove wslcmd_detached
                        &pb_cmd.with_file_name(&wslcmd_detached_filename),
                    )
                    .ok();
                    Err(e) // bypass err
                })
            })
        })
        // refresh wslcmd list if succeeded
        .and_then(|_| {
            self.refresh_wslcmd_list();
            Ok(())
        })
    }

    ///
    /// Unlink an existing WSL command link
    ///
    /// # Arguments
    ///
    /// * `cmdname` - A target command basename to unlink with WSL shell
    ///
    /// # Return
    ///
    /// [`Ok`]\([`()`](unit)\) if succeeded, [`Err`]\([`Error`]\) if failed
    ///
    /// # Examples
    ///
    /// ```
    /// let result = wslcmd_list.unlink_wslcmd("emacs");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn unlink_wslcmd<T: WLPath>(&mut self, cmdname: &T) -> io::Result<()> {
        // create new PathBuf of cmd: replace only filename with cmdname from binpath
        {
            cmdname
                .wlpath_filename() // get filename only, discarding possible parent dir name
                .and_then(|s_cmd| Some(wslcmd_with_ext!(s_cmd))) // append extension to cmdname
                .and_then(|s_file| Some(self.binpath.with_file_name(s_file))) // to abs path
                .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid cmdname"))
        }
        // Ok if valid cmdname
        .and_then(|pb_cmd| {
            self.orig_binpath
                .wlpath_basename()
                .and_then(|s_orig| pb_cmd.wlpath_filename().map(|s_cmd| (s_orig, s_cmd)))
                // bool expression
                .map_or(false, |(s_orig, s_cmd)| {
                    {
                        // cmdname is not the same with orig binname
                        s_orig != s_cmd
                    }
                    .bitand({
                        // cmdname is not detached cmd name pattern (starts with cmdname_delim
                        !s_cmd.starts_with(DETACHED_PROC_PREFIX)
                    })
                })
                // bool -> Result
                .then(|| pb_cmd)
                .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid cmdname"))
        })
        // Ok if given cmd file exists
        .and_then(|pb_cmd| {
            // bool expression
            pb_cmd
                .exists()
                // bool -> Result
                .then(|| pb_cmd)
                .ok_or(Error::new(
                    ErrorKind::AlreadyExists,
                    "WslCmd file does not exist",
                ))
        })
        // Ok if given cmd is wslcmd file
        .and_then(|pb_cmd| {
            // bool expression
            self.is_wslcmd_file(&pb_cmd)
                // bool -> Result
                .then(|| pb_cmd)
                .ok_or(Error::new(
                    ErrorKind::AlreadyExists,
                    "WslCmd already exists for given cmdname",
                ))
        })
        // remove wslcmd symlink
        .and_then(|pb_cmd| {
            let wslcmd_detached_filename = wslcmd_detached_bin!(
                // wslcmd filename
                pb_cmd
                    .wlpath_filename()
                    .ok_or(Error::new(ErrorKind::Other, "Invalid cmdname"))?
            );

            // remove pb_cmd and pb_cmd_detached
            {
                // remove wslcmd (wslcmd -> wslcmd_detached)
                std::fs::remove_file(&pb_cmd)
            }
            .and_then(|_| {
                // remove wslcmd_detached (wslcmd_detached -> orig) if succeeded before
                std::fs::remove_file(pb_cmd.with_file_name(&wslcmd_detached_filename)).or_else(
                    |e| {
                        // if second failed, do restore progress (re-link first removed link)
                        std::os::windows::fs::symlink_file(
                            // target: wslcmd_detached (relative)
                            &wslcmd_detached_filename,
                            // symlink file: wslcmd (absolute)
                            &pb_cmd,
                        )
                        .ok();

                        Err(e) // bypass err
                    },
                )
            })
        })
        // refresh wslcmd list if succeeded
        .and_then(|_| {
            self.refresh_wslcmd_list();
            Ok(())
        })
    }

    // refresh wslcmd list to latest. returns ref of mut self for chaining.
    fn refresh_wslcmd_list(&mut self) -> &mut Self {
        self.get_wslcmd_list_if_changed()
            .and_then(|(cmdlist, cmdlist_time)| {
                self.cmdlist = cmdlist;
                self.cmdlist_time = cmdlist_time;
                Some(())
            });

        self
    }

    // get list of wslcmd only if parent directory is changed
    fn get_wslcmd_list_if_changed(&self) -> Option<(HashSet<String>, Option<SystemTime>)> {
        self.binpath
            // get parent dir
            .wlpath_parent()? // &Path
            // get last modified time
            .metadata()
            .and_then(|md| md.modified())
            .ok()
            // Some(t) if to be refreshed
            .filter(|t_dir| {
                // check if dir mtime is later than the time of cmdlist
                self.cmdlist_time.map_or(true, |t_list| t_dir.gt(&t_list))
            })
            // return tuple (cmdlist, dir_mtime) if to be refreshed
            .map_or(None, |t| Some((self.wslcmd_list(&self.binpath)?, Some(t))))
    }

    // get list of wslcmd from the fs directly
    fn wslcmd_list<T: WLPath>(&self, binpath: &T) -> Option<HashSet<String>> {
        Some(
            binpath
                // get parent dir
                .wlpath_parent()? // &Path
                // get all file list
                .wlpath_read_dir()? // Vec<PathBuf>
                .into_iter()
                // filter files with are only wslcmd
                .filter_map(|pb_f| {
                    self.is_wslcmd_file(&pb_f)
                        .then(|| pb_f.wlpath_basename())
                        .map_or(None, |s| s.wlstr_to_string())
                }) // check if wslcmd
                .collect(),
        )
    }

    // check if given path is wslcmd link
    fn is_wslcmd_file<T: WLPath>(&self, binpath: &T) -> bool {
        // binpath -> (pathbuf_target, pathbuf_followed)
        {
            binpath.wlpath_as_path().and_then(|p| {
                Some((
                    // pb_symlink
                    p.to_path_buf(),
                    // pb_target
                    p.read_link()
                        .map_or(None, |pb| Some(p.with_file_name(pb.wlpath_as_ref()?)))?,
                ))
            })
        }
        // check if wslcmd file using two pathbufs
        .and_then(
            // check if pb_symlink is wslcmd
            |(pb_symlink, pb_target)| {
                // check if both symlink/target pb is symlink to the self bin
                {
                    [&pb_symlink, &pb_target]
                        .iter()
                        .map(|pb| {
                            // bool expression
                            {
                                // OsStr extension == expected bin extension
                                pb.extension().and_then(OsStr::to_str)? == BINARY_EXTENSION
                            }
                            .bitand({
                                // ... and pointing to same bin
                                pb.wlpath_canonicalize()? == self.orig_binpath
                            })
                            .bitand(
                                // ... and not the original bin itself
                                pb.wlpath_filename()? != self.binpath.wlpath_filename()?,
                            )
                            .then(|| ()) // bool to Option
                        })
                        .all(|pred| pred.is_some())
                }
                // check if target is non-detached WslCmd
                .bitand({
                    // bool expression
                    {
                        // not starting with DETACHED_PROC_PREFIX
                        !pb_symlink
                            .wlpath_filename()?
                            .starts_with(DETACHED_PROC_PREFIX)
                    }
                    .bitand(
                        // link behind the current file is detached bin symlink
                        pb_target
                            == pb_target.with_file_name(wslcmd_detached_bin!(
                                pb_symlink.wlpath_filename()?
                            )),
                    )
                })
                .then(|| ()) // final bool to Option
            },
        )
        .is_some()
    }
}

use std::fmt;
/// For display formatted print
impl fmt::Display for WslCmdList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use itertools::Itertools;

        // build string to be displayed
        Some(
            // get new list if self list data is outdated
            self.get_wslcmd_list_if_changed()
                .as_ref()
                // if recent -> self data, if outdated -> new data
                .map_or(&self.cmdlist, |tuple| &tuple.0)
                // convert to basename and sort
                .iter()
                .filter_map(|pb| pb.wlpath_basename())
                .sorted()
                // join all wslcmd into one string
                .join("'    '"), // wrap each cmdname with quotes
        )
        // write only if non-empty
        .filter(|s| !s.is_empty()) // if empty, set to None
        .map_or(Ok(()), |s| write!(f, "'{}'", s)) // write to f if not None
    }
}

#[cfg(test)]
/// For module test
mod test {
    use super::super::{WLPath, WLStr};
    use super::{WslCmdList, BINARY_EXTENSION, DETACHED_PROC_PREFIX};
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::ops::*;
    use std::{collections::HashSet, env, fs, iter::FromIterator, path::PathBuf};

    const TEST_TMP_DIR: &str = "wsllink_tmpdir_test-wslcmd-list_";

    #[derive(Debug)]
    enum TestKind {
        Link,
        Unlink,
    }

    #[test]
    fn test_link() {
        const TMPDIR_POSTFIX: &str = "wslcmd-link";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");
        let (bin1, _) = copy_tmpbin(&tmpdir, None).expect("Bin initialize");

        // test linking only
        let mut wslcmd_list = WslCmdList::new(&bin1).expect("New WslCmdList");
        unit_test_mod(&tmpdir, &mut wslcmd_list, "test", false, TestKind::Link)
            .expect("Test new link: Needs Windows developer mode enabled or admin privilege");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    fn test_unlink() {
        const TMPDIR_POSTFIX: &str = "wslcmd-unlink";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");
        let (bin1, _) = copy_tmpbin(&tmpdir, None).expect("Bin initialize");

        // test linking only
        let mut wslcmd_list = WslCmdList::new(&bin1).expect("New WslCmdList");
        unit_test_mod(&tmpdir, &mut wslcmd_list, "test", false, TestKind::Link)
            .expect("Unlink test prepare");
        unit_test_mod(&tmpdir, &mut wslcmd_list, "test", false, TestKind::Unlink)
            .expect("Test unlink");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    fn test_list() {
        const TMPDIR_POSTFIX: &str = "wslcmd-list";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");
        let (bin1, _) = copy_tmpbin(&tmpdir, None).expect("Bin initialize");
        let dummyfile = tmpdir.join("dummy");

        // test linking only
        let mut wslcmd_list = WslCmdList::new(&bin1).expect("New WslCmdList");
        unit_test_cmdlist(&wslcmd_list, &([] as [&str; 0])); // check when no other file
        unit_test_mod(&tmpdir, &mut wslcmd_list, "testlink", false, TestKind::Link)
            .expect("List test prepare");
        unit_test_cmdlist(&wslcmd_list, &["testlink"]);
        new_dummy_file(&dummyfile); // new dummy file
        unit_test_cmdlist(&wslcmd_list, &["testlink"]);

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    // test all pub funcs
    fn test_overall() {
        const TMPDIR_POSTFIX: &str = "wslcmd-overall";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");
        let (bin1, cmd1) = copy_tmpbin(&tmpdir, None).expect("Bin initialize");
        let (bin2, cmd2) =
            copy_tmpbin(&tmpdir, Some(&format!("bin2-{}", cmd1))).expect("Bin initialize");

        // test with bin1 -
        // validate member funcs
        let mut wslcmd_list = WslCmdList::new(&bin1).expect("New WslCmdList");

        //  - test progress: bin1 -> emacs, t
        unit_test_link_wslcmd(
            &tmpdir,
            &mut wslcmd_list,
            &[("t", false), (&cmd1, true), ("emacs", false), (&cmd2, true)],
        );
        unit_test_cmdlist(&wslcmd_list, &["emacs", "t"]);

        //  - test progress: bin1 -> t
        unit_test_unlink_wslcmd(
            &tmpdir,
            &mut wslcmd_list,
            &[("a", true), ("emacs", false), (&cmd1, true), (&cmd2, true)],
        );
        unit_test_cmdlist(&wslcmd_list, &["t"]);

        //  - test progress: bin1 -> (empty)
        unit_test_unlink_wslcmd(&tmpdir, &mut wslcmd_list, &[("t", false), ("t", true)]);
        unit_test_cmdlist(&wslcmd_list, &([] as [&str; 0]));

        // test with new bin2 -
        // check if bin1 and bin2 in the same dir are working separated on wslcmd_list
        let mut wslcmd2_list = WslCmdList::new(&bin2).expect("New WslCmdList");

        //  - test progress: bin1 -> t2, bin2 -> (empty)
        unit_test_link_wslcmd(&tmpdir, &mut wslcmd_list, &[("t2", false)]);
        unit_test_cmdlist(&wslcmd_list, &["t2"]); // only list for wslcmd_list
        unit_test_cmdlist(&wslcmd2_list, &([] as [&str; 0])); // only list for wslcmd2_list

        //  - test progress: bin1 -> t2, bin2 -> t
        unit_test_link_wslcmd(
            &tmpdir,
            &mut wslcmd2_list,
            &[("t", false), ("t", true), ("t", true)],
        );

        unit_test_cmdlist(&wslcmd_list, &["t2"]); // only list for wslcmd_list
        unit_test_cmdlist(&wslcmd2_list, &["t"]); // only list for wslcmd2_list

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    // clean and then create tmpdir
    fn init_tmpdir(unique_postfix: &str) -> Option<PathBuf> {
        // clean tmpdir if already exists
        clean_tmpdir(unique_postfix);

        // create tmpdir
        Some(env::temp_dir().join(format!("{}{}", TEST_TMP_DIR, unique_postfix)))
            .and_then(|pb| fs::create_dir_all(&pb).ok().map(|_| pb))
    }

    fn copy_tmpbin(tmpdir: &PathBuf, binname: Option<&str>) -> Option<(PathBuf, String)> {
        // get current exe
        env::current_exe()
            .ok()
            // map cur-bin -> (dest-bin, cur-bin)
            .map_or(None, |pb| {
                Some((
                    tmpdir.join(wslcmd_with_ext!(binname.unwrap_or(&pb.wlpath_basename()?))),
                    pb,
                ))
            })
            // copy cur-bin to dest-bin (tmpdir)
            .map_or(None, |(pb_dest, pb_cur)| {
                fs::copy(&pb_cur, &pb_dest).ok()?; // if copy fails, return None
                Some(pb_dest) // pass dest bin path to next map
            })
            // pb_dest to (pb_dest, string_basename)
            .map_or(None, |pb_dest| {
                pb_dest
                    .wlpath_basename()
                    .and_then(|s| s.wlstr_to_string())
                    .map(|s_base| (pb_dest, s_base))
            })
    }

    fn clean_tmpdir(unique_postfix: &str) -> Option<()> {
        Some(std::env::temp_dir().join(format!("{}{}", TEST_TMP_DIR, unique_postfix)))
            .filter(|p| p.exists()) // only if p exists
            .map(|p| {
                match p.is_dir() {
                    true => fs::remove_dir_all(&p),
                    false => fs::remove_file(&p),
                }
                .ok()
            })?
    }

    fn unit_test_link_wslcmd(
        tmpdir: &PathBuf,
        wslcmd_list: &mut WslCmdList,
        cmdname_and_shoulderr: &[(&str, bool)],
    ) {
        for cur_elem in cmdname_and_shoulderr {
            unit_test_mod(tmpdir, wslcmd_list, cur_elem.0, cur_elem.1, TestKind::Link)
                .expect(&format!("unit_test_link_wslcmd(\"{}\")", cur_elem.0))
        }
    }

    fn unit_test_unlink_wslcmd(
        tmpdir: &PathBuf,
        wslcmd_list: &mut WslCmdList,
        cmdname_and_shoulderr: &[(&str, bool)],
    ) {
        for cur_elem in cmdname_and_shoulderr {
            unit_test_mod(
                tmpdir,
                wslcmd_list,
                cur_elem.0,
                cur_elem.1,
                TestKind::Unlink,
            )
            .expect(&format!("unit_test_unlink_wslcmd(\"{}\")", cur_elem.0))
        }
    }

    fn unit_test_cmdlist<T: WLStr>(wslcmd_list: &WslCmdList, expected_result: &[T]) {
        unit_test_list(wslcmd_list, expected_result).expect("unit_test_cmdlist()")
    }

    fn unit_test_mod(
        tmpdir: &PathBuf,
        wslcmd_list: &mut WslCmdList,
        cmdname: &str,
        should_err: bool,
        testkind: TestKind,
    ) -> io::Result<()> {
        dbg!(&tmpdir, &wslcmd_list, &cmdname, &should_err, &testkind);

        // get dir entries before call
        let dirent_before: HashSet<_> = HashSet::from_iter(
            tmpdir
                .wlpath_read_dir()
                .ok_or(Error::new(ErrorKind::Other, "Error on prepare"))?,
        );

        // run link_wslcmd
        let call_result = match testkind {
            TestKind::Link => wslcmd_list.link_wslcmd(&cmdname),
            TestKind::Unlink => wslcmd_list.unlink_wslcmd(&cmdname),
        };
        dbg!(&call_result);

        // get dir entries after call
        let dirent_after: HashSet<_> = HashSet::from_iter(
            tmpdir
                .wlpath_read_dir()
                .ok_or(Error::new(ErrorKind::Other, "Error on prepare"))?,
        );

        // get diff of before <-> after
        let mut dirent_diff = dirent_before.symmetric_difference(&dirent_after);

        // validate result if worked as expected
        match should_err {
            // if the call result should be Err
            true => {
                // check if Err
                {
                    call_result.is_err().then(|| ()).ok_or(Error::new(
                        ErrorKind::Other,
                        concat!(
                            "Test validation failed: ",
                            "Function call returned Ok while it should return Err"
                        ),
                    ))
                }
                // check if changes are as expected
                .and_then(|_|
                    // check if no difference before & after the func call
                    (dirent_diff.by_ref().count() == 0)
                        .then(|| ())
                        .ok_or(Error::new(
                            ErrorKind::Other,
                            concat!(
                                "Test validation failed: ",
                                "Directory entries changed after failed job"
                            ),
                        )))
            }

            // if the call result should be Ok
            false => {
                // check if Ok
                {
                    call_result.is_ok().then(|| ()).ok_or(Error::new(
                        ErrorKind::Other,
                        concat!(
                            "Test validation failed: ",
                            "Function call returned Err while it should return Ok"
                        ),
                    ))
                }
                // check if changes are as expected
                .and_then(|_| {
                    // check if # increased when Link, and decreased when Unlink
                    match testkind {
                        TestKind::Link => dirent_before.len() < dirent_after.len(),
                        TestKind::Unlink => dirent_before.len() > dirent_after.len(),
                    }
                    // check if different item is what the call expected
                    .bitand({
                        let diff_expected = [
                            wslcmd_list
                                .binpath
                                .with_file_name(wslcmd_with_ext!(cmdname)),
                            wslcmd_list
                                .binpath
                                .with_file_name(wslcmd_detached_bin!(wslcmd_with_ext!(cmdname))),
                        ];

                        HashSet::<_>::from_iter(&diff_expected)
                            .symmetric_difference(&HashSet::from_iter(&mut dirent_diff))
                            .count()
                            == 0
                    })
                    .then(|| ())
                    .ok_or(Error::new(
                        ErrorKind::Other,
                        concat!(
                            "Test validation failed: ",
                            "Changed directory entries are not matched with expected changes"
                        ),
                    ))
                })
            }
        }
    }

    fn unit_test_list<T: WLStr>(wslcmd_list: &WslCmdList, expected_result: &[T]) -> io::Result<()> {
        // convert cmdlist in wslcmd_list to basenamed result
        let cmdlist_basename: HashSet<_> =
            HashSet::from_iter(wslcmd_list.cmdlist().iter().map(|pb| pb.wlpath_basename()));
        //dbg!(&cmdlist_basename);

        // convert expected list to basenamed result
        let expected_list_basename: HashSet<_> =
            HashSet::from_iter(expected_result.iter().map(|s| s.wlpath_basename()));
        //dbg!(&expected_list_basename);

        // get diff of expected and real result
        let list_diff = cmdlist_basename.symmetric_difference(&expected_list_basename);
        //dbg!(&list_diff);

        // validate result if worked as expected
        { list_diff.count() == 0 }
            .then(|| ())
            .ok_or(Error::new(ErrorKind::Other, "Test validation failed"))
    }

    fn new_dummy_file<T: WLPath>(fpath: &T) {
        use std::fs::File;
        use std::io::prelude::*;

        fpath
            .wlpath_as_ref()
            .ok_or(Error::new(ErrorKind::InvalidInput, "Arguments not valid"))
            .and_then(|s| File::create(s))
            .and_then(|mut f| f.write_all(b"wsllink dummy"))
            .expect("Dummy file creation");
    }
}
