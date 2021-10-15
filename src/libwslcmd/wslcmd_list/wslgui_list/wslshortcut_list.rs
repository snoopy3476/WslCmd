// crate
use super::wcstr;
use wcstr::*;

// general
use lnk::ShellLink;
use std::collections::HashSet;
use std::io;
use std::io::Error;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
/// Read dir and load all wslshortcuts, and manage the list
pub struct WslShortcutList {
    /// Path of target directory
    dirpath: PathBuf,

    /// Latest WSL shortcut list
    shortcut_list_cached: HashSet<String>,

    /// Time of WSL shortcut list data
    shortcut_list_cached_time: Option<SystemTime>,
}

impl WslShortcutList {
    ///
    /// Create new [`WslShortcutList`]
    ///
    /// # Arguments
    ///
    /// * `dirpath` - A target directory path to get list of wslshortcut
    ///
    /// # Return
    ///
    /// A newly created [`Some`]\([`WslShortcutList`]\), initialized with wslshortcut list.
    ///
    /// [`None`] if failed to initialize [`WslShortcutList`].
    ///
    /// # Examples
    ///
    /// ```
    /// let wslshortcut_list = WslShortcutList::new(&"/path/to/target/dir");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn new<T: WCPath>(dirpath: &T) -> Option<Self> {
        // initialize basic info
        let dirpath = dirpath.wcpath_clone_to_pathbuf()?;

        // build struct instance
        Some(Self {
            dirpath,
            shortcut_list_cached: HashSet::new(), // dummy
            shortcut_list_cached_time: None,      // dummy
        })
    }

    ///
    /// Add a new shortcut to WSL command
    ///
    /// # Arguments
    ///
    /// * `cmdname` - A target command basename to add
    ///
    /// # Return
    ///
    /// [`Ok`]\([`()`](unit)\) if succeeded, [`Err`]\([`Error`]\) if failed
    ///
    /// # Examples
    ///
    /// ```
    /// let result = wslshortcut_list.add_wslshortcut("emacs");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn add_wslshortcut<T: WCStr>(&mut self, cmdname: &T) -> io::Result<()> {
        // create new PathBuf of shortcut
        ShellLink::new_simple(std::path::Path::new(r"C:\Windows\System32\notepad.exe"))?;

        Ok(())
    }

    ///
    /// Remove an existing shortcut to WSL command
    ///
    /// # Arguments
    ///
    /// * `cmdname` - A target command basename to remove
    ///
    /// # Return
    ///
    /// [`Ok`]\([`()`](unit)\) if succeeded, [`Err`]\([`Error`]\) if failed
    ///
    /// # Examples
    ///
    /// ```
    /// let result = wslshortcut_list.remove_wslshortcut("emacs");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn remove_wslshortcut<T: WCStr>(&mut self, cmdname: &T) -> io::Result<()> {
        Ok(())
    }

    // refresh wslshortcut list to latest. returns ref of mut self for chaining.
    fn refresh_wslshortcut_list(&mut self) -> &mut Self {
        self.get_wslshortcut_list_if_changed()
            .and_then(|(shortcut_list, shortcut_list_time)| {
                self.shortcut_list_cached = shortcut_list;
                self.shortcut_list_cached_time = shortcut_list_time;
                Some(())
            });

        self
    }

    // get list of wslshortcut only if parent directory is changed
    fn get_wslshortcut_list_if_changed(&self) -> Option<(HashSet<String>, Option<SystemTime>)> {
        self.dirpath
            // get last modified time
            .metadata()
            .and_then(|md| md.modified())
            .ok()
            // Some(t) if to be refreshed
            .filter(|t_dir| {
                // check if dir mtime is later than the time of shortcut list
                self.shortcut_list_cached_time
                    .map_or(true, |t_list| t_dir.gt(&t_list))
            })
            // return tuple (wslshortcut, dir_mtime) if to be refreshed
            .map_or(None, |t| Some((self.wslshortcut_list()?, Some(t))))
    }

    // get list of wslshortcut from the fs directly
    fn wslshortcut_list(&self) -> Option<HashSet<String>> {
        Some(
            self.dirpath
                // get all file list
                .wcpath_read_dir()? // Vec<PathBuf>
                .into_iter()
                // filter files with are only wslshortcut
                .filter_map(|pb_f| {
                    self.is_wslshortcut_file(&pb_f)
                        .then(|| pb_f.wcpath_fstem())
                        .map_or(None, |s| s.wcstr_to_string())
                }) // check if wslshortcut
                .collect(),
        )
    }

    // check if given path is wslshortcut
    fn is_wslshortcut_file<T: WCPath>(&self, filepath: &T) -> bool {
        true
    }
}

#[cfg(test)]
/// For module test
mod test {
    use super::WslShortcutList;
    use super::{WCPath, WCStr};
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::ops::*;
    use std::{collections::HashSet, env, fs, iter::FromIterator, path::PathBuf};

    const TEST_TMP_DIR: &str = "wslcmd_tmpdir_test-wslshortcut-list_";

    #[test]
    fn test_add() {
        const TMPDIR_POSTFIX: &str = "wslshortcut-add";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    fn test_remove() {
        const TMPDIR_POSTFIX: &str = "wslshortcut-remove";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    fn test_list() {
        const TMPDIR_POSTFIX: &str = "wslshortcut-list";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    // test all pub funcs
    fn test_overall() {
        const TMPDIR_POSTFIX: &str = "wslshortcut-overall";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");
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

    fn new_dummy_file<T: WCPath>(fpath: &T) {
        use std::fs::File;
        use std::io::prelude::*;

        fpath
            .wcpath_as_path()
            .ok_or(Error::new(ErrorKind::InvalidInput, "Arguments not valid"))
            .and_then(|p| File::create(p))
            .and_then(|mut f| f.write_all(b"wslcmd dummy"))
            .expect("Dummy file creation");
    }
}
