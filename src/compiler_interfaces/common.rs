// src/compiler_interfaces/common.rs
use std::{env, io};
use std::fs;
use std::path::PathBuf;

use glob::glob;
use serde_json::to_string;
use crate::project::Project;

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

pub fn list_files(path: &PathBuf) -> io::Result<Vec<PathBuf>> {

    env::set_current_dir(path)?;
    
    let mut vec = Vec::new();
    let dir_path= "./*";

    for e in glob(&dir_path).expect("Failed to read glob pattern") {
        match e {
            Ok(p) => vec.push(p.to_path_buf()),
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }

    //let path = path.into();
    //_list_files(&mut vec, path)?;
    Ok(vec)
}

// TRAITS DEFINITIONS
pub trait Compiler {
    fn compile_file(&self, source_dir: &Path, project_path: &String, rel_file_path: &String) -> Result<(), &'static str>;
    fn compile_project(&self, project: Project, project_path: PathBuf, working_dir: PathBuf) -> Result<(), &'static str>;
    fn detect_compiler_path() -> Option<String>;
}
