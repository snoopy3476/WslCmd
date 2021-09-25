use derive_getters::Getters;

use super::WLPath;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Getters, Debug)]
/// Read dir and load all wslcmds, and manage the list
pub struct WslCmdList {
    /// For preventing direct struct creating
    #[getter(skip)]
    _no_direct_construct: (),

    /// Path of target bin
    binpath: PathBuf,

    /// Latest WslCmd list
    cmdlist: HashSet<PathBuf>,

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
    /// A newly created [`WslCmdList`], initialized with wslcmd list
    ///
    /// # Examples
    ///
    /// ```
    /// let wslcmd_list: WslCmdList = WslCmdList::new(&"/path/to/target/exe");
    /// ```
    ///
    pub fn new<T: WLPath>(binpath: &T) -> Option<Self> {
        let binpath = binpath.wlpath_clone_to_pathbuf()?;
        let (cmdlist, cmdlist_time) = Self::get_wslcmd_list_if_changed(&binpath, None)?;

        // return struct instance
        Some({
            Self {
                _no_direct_construct: (),
                binpath,
                cmdlist,
                cmdlist_time,
            }
        })
    }

    // refresh wslcmd list to latest
    fn refresh_wslcmd_list(&mut self) {
        Self::get_wslcmd_list_if_changed(&self.binpath, None).map(|(cmdlist, cmdlist_time)| {
            self.cmdlist = cmdlist;
            self.cmdlist_time = cmdlist_time;
        });
    }

    // get list of wslcmd only if parent directory is changed
    fn get_wslcmd_list_if_changed<T: WLPath>(
        binpath: &T,
        cmdlist_time: Option<SystemTime>,
    ) -> Option<(HashSet<PathBuf>, Option<SystemTime>)> {
        binpath
            // get parent dir
            .wlpath_parent()? // &Path
            // get last modified time
            .metadata()
            .and_then(|md| md.modified())
            .ok()
            // Some(t) if to be refreshed
            .filter(|t_dir| {
                // check if dir mtime is later than the time of cmdlist
                cmdlist_time.map_or(true, |t_list| t_dir.gt(&t_list))
            })
            // return tuple (cmdlist, dir_mtime) if to be refreshed
            .map_or(None, |t| Some((Self::wslcmd_list(binpath)?, Some(t))))
    }

    // get list of wslcmd
    fn wslcmd_list<T: WLPath>(binpath: &T) -> Option<HashSet<PathBuf>> {
        let binname = binpath.wlpath_basename()?;
        let orig_binpath = binpath.wlpath_canonicalize()?;

        Some(
            binpath
                // get parent dir
                .wlpath_parent()? // &Path
                // get all file list
                .wlpath_read_dir()? // Vec<PathBuf>
                .into_iter()
                // filter files with are only wslcmd
                .filter(
                    // check if wslcmd
                    |pb_f| {
                        // if pointing to same bin
                        pb_f.extension() == Some(OsStr::new("exe"))
                        // pointing to same bin
                            && pb_f.wlpath_canonicalize().map_or(false, |pb| pb == orig_binpath)
                        // not bin itself
                            && pb_f.wlpath_basename().map_or(false, |s| s != binname)
                    },
                )
                .collect(),
        )
    }

    pub fn push(&mut self, input: PathBuf) {
        self.cmdlist.insert(input);
    }
}

use std::fmt;
/// For display formatted print
impl fmt::Display for WslCmdList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format
        write!(
            f,
            "\"{}\"",
            // get new list if self list data is outdated
            Self::get_wslcmd_list_if_changed(&self.binpath, self.cmdlist_time)
                .as_ref()
                // if recent -> self data, if outdated -> new data
                .map_or(&self.cmdlist, |tuple| &tuple.0)
                // convert to basename
                .iter()
                .filter_map(|pb| pb.wlpath_basename())
                // join all wslcmd into one string
                .collect::<Vec<&str>>()
                .join("\"    \"") // wrap each cmdname with quotes
        )
    }
}
