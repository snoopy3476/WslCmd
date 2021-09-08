use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

mod common;
mod execute;
mod manage;

use common::PathTrait;

fn main() {
    let args: Vec<String> = env::args().collect();
    let orig_bin_basename: String = get_binary_name();
    let cmd_bin_basename: String = file_basename(Path::new(&args[0]).to_path_buf());

    if orig_bin_basename != cmd_bin_basename {
        println!("Exec mode!");
        execute::execute_mode(&orig_bin_basename, &cmd_bin_basename, &args)
    } else {
        println!("Mgmt mode!");
        manage::manage_mode(&orig_bin_basename, &cmd_bin_basename, &args)
    }
}

fn get_binary_name() -> String {
    std::env::current_exe() // Result<PathBuf> link_or_bin_fullpath
        .ok() // ... -> Option<PathBuf>
        .map(deref_link_chains) // Option<PathBuf> bin_fullpath
        .map_or(String::new(), file_basename) // ... -> String bin_basename
}

fn file_basename<T: PathTrait>(path: T) -> String {
    path.basename() // PathBuf path -> Option<&OsStr> basename
        .map_or(None, OsStr::to_str) // ... -> Option<&str>
        .map_or(String::new(), String::from) // ... -> String
}

fn deref_link_chains<T: PathTrait>(path: T) -> PathBuf {
    path.deref_link() // deref link
        .map_or(path.to_path_buf(), deref_link_chains) // if deref failed, return cur path
}
