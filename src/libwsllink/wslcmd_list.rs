use derive_getters::Getters;

use super::{WLPath, WLStr};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::io;
use std::io::{Error, ErrorKind};
use std::ops::*;
use std::path::PathBuf;
use std::time::SystemTime;

/// Extension of Windows binary
const BINARY_EXTENSION: &str = "exe";

macro_rules! wslcmd_with_ext {
    ($label:expr) => {
        [$label, ".", BINARY_EXTENSION].concat()
    };
}

#[derive(Getters, Debug)]
/// Read dir and load all wslcmds, and manage the list
pub struct WslCmdList {
    /// Path of target bin
    binpath: PathBuf,

    /// Path of original bin, after following all symlinks
    orig_binpath: PathBuf,

    /// Latest WslCmd list
    cmdlist: HashSet<String>,

    /// Time of WslCmd list data
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
        // replace only filename with cmdname from binpath
        Some(self.binpath.with_file_name({
            wslcmd_with_ext!(cmdname
                // get filename
                .wlpath_filename()
                // check if cmdname is same as orig binname
                .filter(|s_wslcmd| {
                    self.orig_binpath
                        .wlpath_basename()
                        .map_or(false, |s_orig| &s_orig != s_wslcmd)
                })
                // Option -> Result
                .ok_or(Error::new(
                    ErrorKind::InvalidInput,
                    "link_wslcmd(): Invalid cmdname",
                ))?)
        }))
        // create new symlink
        .map_or(
            Err(Error::new(ErrorKind::Other, "Should not be reached")),
            |pb| {
                std::os::windows::fs::symlink_file(
                    &self.binpath, // link origpath
                    pb,            // link file
                )
            },
        )
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
        // replace only filename with cmdname from self binpath,
        Some(self.binpath.with_file_name({
            wslcmd_with_ext!(cmdname
                // get filename
                .wlpath_filename()
                // check if cmdname is same as orig binname
                .filter(|s_wslcmd| self
                    .orig_binpath
                    .wlpath_basename()
                    .map_or(false, |s_orig| &s_orig != s_wslcmd))
                // Option -> Result
                .ok_or(Error::new(
                    ErrorKind::InvalidInput,
                    "link_wslcmd(): Invalid cmdname",
                ))?)
        }))
        // check if wslcmd file
        .filter(|pb| self.is_wslcmd_file(pb))
        // remove wslcmd symlink
        .map_or(
            Err(Error::new(
                ErrorKind::NotFound,
                "unlink_wslcmd(): WslCmd not found for given cmdname",
            )),
            |pb| std::fs::remove_file(self.binpath.with_file_name(pb)),
        )
        // refresh wslcmd list if succeeded
        .and_then(|_| {
            self.refresh_wslcmd_list();
            Ok(())
        })
    }

    // refresh wslcmd list to latest. returns ref of mut self for chaining.
    fn refresh_wslcmd_list(&mut self) -> &mut Self {
        self.get_wslcmd_list_if_changed()
            .map(|(cmdlist, cmdlist_time)| {
                self.cmdlist = cmdlist;
                self.cmdlist_time = cmdlist_time;
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

    // get list of wslcmd
    fn wslcmd_list<T: WLPath>(&self, binpath: &T) -> Option<HashSet<String>> {
        Some(
            binpath
                // get parent dir
                .wlpath_parent()? // &Path
                // get all file list
                .wlpath_read_dir()? // Vec<PathBuf>
                .into_par_iter()
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
        binpath.wlpath_clone_to_pathbuf().map_or(false, {
            // check if pb_file is wslcmd
            |pb_file| {
                // file with BINARY_EXTENSION extension
                {
                    (
                        // OsStr extension
                        (pb_file.extension().and_then(OsStr::to_str))
                    ) == (
                        // expected bin extension
                        Some(BINARY_EXTENSION)
                    )
                }
                // ... and pointing to same bin
                .bitand(
                    pb_file
                        .wlpath_canonicalize()
                        .map_or(false, |pb_file_orig| pb_file_orig == self.orig_binpath),
                )
                // ... and not the original bin itself
                .bitand(pb_file.wlpath_basename().map_or(false, |s| {
                    s != self.binpath.wlpath_basename().unwrap_or_default()
                }))
            }
        })
    }
}

use std::fmt;
/// For display formatted print
impl fmt::Display for WslCmdList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // build string to be displayed
        Some(
            // get new list if self list data is outdated
            self.get_wslcmd_list_if_changed()
                .as_ref()
                // if recent -> self data, if outdated -> new data
                .map_or(&self.cmdlist, |tuple| &tuple.0)
                // convert to basename
                .par_iter()
                .filter_map(|pb| pb.wlpath_basename())
                // join all wslcmd into one string
                .collect::<Vec<&str>>()
                .join("\"    \""), // wrap each cmdname with quotes
        )
        // write only if non-empty
        .filter(|s| !s.is_empty()) // if empty, set to None
        .map_or(Ok(()), |s| write!(f, "\"{}\"", s)) // write to f if not None
    }
}

#[cfg(test)]
/// For module test
mod test {
    use super::super::{WLPath, WLStr};
    use super::{WslCmdList, BINARY_EXTENSION};
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::ops::*;
    use std::{collections::HashSet, env, fs, iter::FromIterator, path::PathBuf};

    const TEST_TMP_DIR: &str = "wsllink_tmpdir_test-wslcmd-list_";

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
            copy_tmpbin(&tmpdir, Some(&["bin2-", cmd1.as_str()].concat())).expect("Bin initialize");

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
        Some(env::temp_dir().join([TEST_TMP_DIR, unique_postfix].concat()))
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
        Some(std::env::temp_dir().join([TEST_TMP_DIR, unique_postfix].concat()))
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
                .expect(&["unit_test_link_wslcmd(\"", cur_elem.0, "\")"].concat())
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
            .expect(&["unit_test_unlink_wslcmd(\"", cur_elem.0, "\")"].concat())
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
        // get dir entries before call
        let dirent_before: HashSet<_> = HashSet::from_iter(
            tmpdir
                .wlpath_read_dir()
                .ok_or(Error::new(ErrorKind::Other, "Error on prepare"))?,
        );

        // run link_wslcmd
        dbg!(match testkind {
            TestKind::Link => wslcmd_list.link_wslcmd(&cmdname),
            TestKind::Unlink => wslcmd_list.unlink_wslcmd(&cmdname),
        })
        .ok(); // remove unused warning

        // get dir entries after call
        let dirent_after: HashSet<_> = HashSet::from_iter(
            tmpdir
                .wlpath_read_dir()
                .ok_or(Error::new(ErrorKind::Other, "Error on prepare"))?,
        );

        // get diff of before <-> after
        let dirent_diff = dirent_before
            .symmetric_difference(&dirent_after)
            .enumerate();

        // validate result if worked as expected
        match {
            // check if: Link -> len increased, Unlink -> len decreased
            match testkind {
                TestKind::Link => dirent_before.len() < dirent_after.len(),
                TestKind::Unlink => dirent_before.len() > dirent_after.len(),
            }
            // check if: diff count is 1, and different item is what the call expected
            .bitand(dirent_diff.last().map_or(false, |(i, pb)| {
                pb.wlpath_filename().map_or(false, |s| {
                    {
                        // check if diff count is 1 (last elem idx is 0)
                        i == 0
                    }
                    .bitand({
                        // check if diff elem is same as expected
                        s == wslcmd_with_ext!(cmdname)
                    })
                })
            }))
            // if should_err is true, followed bool result will be inverted
            .bitxor(should_err)
        } {
            true => Ok(()),
            false => Err(Error::new(ErrorKind::Other, "Test validation failed")),
        }
    }

    fn unit_test_list<T: WLStr>(wslcmd_list: &WslCmdList, expected_result: &[T]) -> io::Result<()> {
        // convert cmdlist in wslcmd_list to basenamed result
        let cmdlist_basename: HashSet<_> =
            HashSet::from_iter(wslcmd_list.cmdlist().iter().map(|pb| pb.wlpath_basename()));

        // convert expected list to basenamed result
        let expected_list_basename: HashSet<_> =
            HashSet::from_iter(expected_result.iter().map(|s| s.wlpath_basename()));

        // get diff of expected and real result
        let list_diff = cmdlist_basename.symmetric_difference(&expected_list_basename);

        // validate result if worked as expected
        match list_diff.count() == 0 {
            true => Ok(()),
            false => Err(Error::new(ErrorKind::Other, "Test validation failed")),
        }
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
