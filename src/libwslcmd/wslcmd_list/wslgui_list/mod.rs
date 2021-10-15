// crate
/// Manage list of WSL shortcut
mod wslshortcut_list;
use wslshortcut_list::WslShortcutList;
/// Manage list of Windows associated programs for WSL
mod wslassoc_list;
use super::wcstr;
use wcstr::*;
use wslassoc_list::WslAssocList;

// general
use std::collections::HashSet;
use std::io;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug)]
/// Read dir and load all wslguis, and manage the list
pub struct WslGuiList {
    /// Path of target directory
    dirpath: PathBuf,

    /// List of WSL shortcut
    shortcut_list: WslShortcutList,

    /// List of associated programs
    assoc_list: WslAssocList,
}

impl WslGuiList {
    ///
    /// Create new [`WslGuiList`]
    ///
    /// # Arguments
    ///
    /// * `dirpath` - A target directory path to get list of wslgui
    ///
    /// # Return
    ///
    /// A newly created [`Some`]\([`WslGuiList`]\), initialized with wslgui list.
    ///
    /// [`None`] if failed to initialize [`WslGuiList`].
    ///
    /// # Examples
    ///
    /// ```
    /// let wslgui_list = WslGuiList::new(&"/path/to/target/dir");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn new<T: WCPath>(dirpath: &T) -> Option<Self> {
        // build struct instance
        Some(Self {
            dirpath: dirpath.wcpath_clone_to_pathbuf()?,
            shortcut_list: WslShortcutList::new(dirpath)?,
            assoc_list: WslAssocList::new(dirpath)?,
        })
    }

    ///
    /// Add a new gui to WSL command
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
    /// let result = wslgui_list.add_wslgui("emacs");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn add_wslgui<T: WCStr>(&mut self, cmdname: &T) -> io::Result<()> {
        self.shortcut_list.add_wslshortcut(cmdname).and_then(|_| {
            self.assoc_list
                .add_wslassoc(cmdname)
                .or_else(|_| self.shortcut_list.remove_wslshortcut(cmdname))
        })
    }

    ///
    /// Remove an existing gui to WSL command
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
    /// let result = wslgui_list.remove_wslgui("emacs");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn remove_wslgui<T: WCStr>(&mut self, cmdname: &T) -> io::Result<()> {
        Ok(())
    }

    // get list of wslgui from the fs directly
    fn wslgui_list<T: WCPath>(&self, dirpath: &T) -> Option<HashSet<String>> {
        None
    }
}

#[cfg(test)]
/// For module test
mod test {
    use super::WslGuiList;
    use super::{WCPath, WCStr};
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::ops::*;
    use std::{collections::HashSet, env, fs, iter::FromIterator, path::PathBuf};

    const TEST_TMP_DIR: &str = "wslcmd_tmpdir_test-wslgui-list_";

    #[test]
    fn test_add() {
        const TMPDIR_POSTFIX: &str = "wslgui-add";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    fn test_remove() {
        const TMPDIR_POSTFIX: &str = "wslgui-remove";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    fn test_list() {
        const TMPDIR_POSTFIX: &str = "wslgui-list";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    // test all pub funcs
    fn test_overall() {
        const TMPDIR_POSTFIX: &str = "wslgui-overall";

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
