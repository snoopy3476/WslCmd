// crate
use super::wcstr::*;

// general
use std::collections::HashSet;
use std::io;
use std::io::Error;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
/// Read dir and load all wslassocs, and manage the list
pub struct WslAssocList {
    /// Path of target directory
    dirpath: PathBuf,

    /// Latest WSL assoc list
    assoc_list_cached: HashSet<String>,

    /// Time of WSL assoc list data
    assoc_list_cached_time: Option<SystemTime>,
}

impl WslAssocList {
    ///
    /// Create new [`WslAssocList`]
    ///
    /// # Arguments
    ///
    /// * `dirpath` - A target directory path to get list of wslassoc
    ///
    /// # Return
    ///
    /// A newly created [`Some`]\([`WslAssocList`]\), initialized with wslassoc list.
    ///
    /// [`None`] if failed to initialize [`WslAssocList`].
    ///
    /// # Examples
    ///
    /// ```
    /// let wslassoc_list = WslAssocList::new(&"/path/to/target/dir");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn new<T: WCPath>(dirpath: &T) -> Option<Self> {
        // initialize basic info
        let dirpath = dirpath.wcpath_clone_to_pathbuf()?;

        // build struct instance
        Some(Self {
            dirpath,
            assoc_list_cached: HashSet::new(), // dummy
            assoc_list_cached_time: None,      // dummy
        })
    }

    ///
    /// Add a new assoc to WSL command
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
    /// let result = wslassoc_list.add_wslassoc("emacs");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn add_wslassoc<T: WCStr>(&mut self, cmdname: &T) -> io::Result<()> {
        // registry
        use winreg::enums::*;
        use winreg::RegKey;

        RegKey::predef(HKEY_CURRENT_USER).create_subkey(r"Software\Classes\WslCmd")?;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let src = hkcu.open_subkey_with_flags(r"Software\Classes", KEY_READ)?;
        let (dst, dst_disp) = hkcu.create_subkey(r"Software\Classes\.bmptest")?;
        src.copy_tree(".bmp", &dst)?;

        sh_change_notify_assoc();

        Ok(())
    }

    ///
    /// Remove an existing assoc to WSL command
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
    /// let result = wslassoc_list.remove_wslassoc("emacs");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn remove_wslassoc<T: WCStr>(&mut self, cmdname: &T) -> io::Result<()> {
        Ok(())
    }

    // refresh wslassoc list to latest. returns ref of mut self for chaining.
    fn refresh_wslassoc_list(&mut self) -> &mut Self {
        self.get_wslassoc_list_if_changed()
            .and_then(|(assoc_list, assoc_list_time)| {
                self.assoc_list_cached = assoc_list;
                self.assoc_list_cached_time = assoc_list_time;
                Some(())
            });

        self
    }

    // get list of wslassoc only if parent directory is changed
    fn get_wslassoc_list_if_changed(&self) -> Option<(HashSet<String>, Option<SystemTime>)> {
        self.dirpath
            // get last modified time
            .metadata()
            .and_then(|md| md.modified())
            .ok()
            // Some(t) if to be refreshed
            .filter(|t_dir| {
                // check if dir mtime is later than the time of assoc list
                self.assoc_list_cached_time
                    .map_or(true, |t_list| t_dir.gt(&t_list))
            })
            // return tuple (wslassoc, dir_mtime) if to be refreshed
            .map_or(None, |t| Some((self.wslassoc_list()?, Some(t))))
    }

    // get list of wslassoc from the fs directly
    fn wslassoc_list(&self) -> Option<HashSet<String>> {
        Some(
            self.dirpath
                // get all file list
                .wcpath_read_dir()? // Vec<PathBuf>
                .into_iter()
                // filter files with are only wslassoc
                .filter_map(|pb_f| {
                    self.is_wslassoc_file(&pb_f)
                        .then(|| pb_f.wcpath_fstem())
                        .map_or(None, |s| s.wcstr_to_string())
                }) // check if wslassoc
                .collect(),
        )
    }

    // check if given path is wslassoc
    fn is_wslassoc_file<T: WCPath>(&self, filepath: &T) -> bool {
        true
    }
}

// Notify assoc change by SHChangeNotify
fn sh_change_notify_assoc() {
    use std::ptr::null_mut;
    use winapi::shared::minwindef::{LPCVOID, UINT};
    use winapi::um::winnt::LONG;

    // define dylib fn
    #[link(name = "shell32", kind = "dylib")]
    extern "C" {
        fn SHChangeNotify(wEventId: LONG, uFlags: UINT, dwItem1: LPCVOID, dwItem2: LPCVOID);
    }

    // Notify the shell
    const SHCNE_ASSOCCHANGED: LONG = 0x08000000;
    const SHCNF_IDLIST: UINT = 0;
    unsafe {
        SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, null_mut(), null_mut());
    }
}

#[cfg(test)]
/// For module test
mod test {
    use super::WslAssocList;
    use super::{WCPath, WCStr};
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::ops::*;
    use std::{collections::HashSet, env, fs, iter::FromIterator, path::PathBuf};

    const TEST_TMP_DIR: &str = "wslcmd_tmpdir_test-wslassoc-list_";

    #[test]
    fn test_add() {
        const TMPDIR_POSTFIX: &str = "wslassoc-add";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    fn test_remove() {
        const TMPDIR_POSTFIX: &str = "wslassoc-remove";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    fn test_list() {
        const TMPDIR_POSTFIX: &str = "wslassoc-list";

        // init tmpdir
        let tmpdir = init_tmpdir(TMPDIR_POSTFIX).expect("Tmp dir initialize");

        // clean tmpdir
        clean_tmpdir(TMPDIR_POSTFIX);
    }

    #[test]
    // test all pub funcs
    fn test_overall() {
        const TMPDIR_POSTFIX: &str = "wslassoc-overall";

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
