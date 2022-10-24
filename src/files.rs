use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub(crate) fn get_statm(path: PathBuf) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

pub(crate) fn get_stat(path: PathBuf) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

pub(crate) fn get_status(path: PathBuf) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}