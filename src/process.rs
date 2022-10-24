use std::io;
use std::path::{Path, PathBuf};

use crate::files;

pub fn process_exists(pid: u32) -> Option<PathBuf> {
    let path_str = format!("/proc/{}/", pid);
    let path = Path::new(&path_str);
    if path.is_dir() {
        return Some(path.into());
    }
    None
}

pub fn read_process(path: PathBuf) -> io::Result<()> {
    let statm_path = path.clone().join("statm");
    files::get_statm(statm_path)?;

    let stat_path = path.clone().join("stat");
    files::get_stat(stat_path)?;

    let status_path = path.clone().join("status");
    files::get_status(status_path)?;

    Ok(())
}