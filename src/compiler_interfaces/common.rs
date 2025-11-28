// src/compiler_interfaces/common.rs
use std::io;
use std::fs;
use std::path::PathBuf;

use glob::glob;

fn _list_files(vec: &mut Vec<PathBuf>, path: PathBuf) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let full_path = entry?.path();
            _list_files(vec, full_path)?;
        }
    } else if path.is_file() {
        vec.push(path);
    }
    Ok(())
}

pub fn list_files<T: Into<PathBuf>>(path: T) -> io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();

    for e in glob("./*").expect("Failed to read glob pattern") {
        match e {
            Ok(p) => vec.push(p.to_path_buf()),
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }

    let path = path.into();
    _list_files(&mut vec, path)?;
    Ok(vec)
}
