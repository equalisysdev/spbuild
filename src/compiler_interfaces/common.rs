// src/compiler_interfaces/common.rs
use std::{io};
use std::fs;
use std::path::PathBuf;

use crate::project::Project;

fn _list_files(vec: &mut Vec<PathBuf>, path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let full_path = entry?.path();
            _list_files(vec, &full_path)?;
        }
    } else if path.is_file() {
        vec.push(path.to_path_buf());
    }
    Ok(())
}

pub fn list_files(root: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut abs_files = Vec::new();
    _list_files(&mut abs_files, root)?;

    // Convert to paths relative to `root` so callers can safely join(root, rel).
    let mut rel_files = Vec::with_capacity(abs_files.len());
    for p in abs_files {
        match p.strip_prefix(root) {
            Ok(rel) => rel_files.push(rel.to_path_buf()),
            Err(_) => rel_files.push(p),
        }
    }

    Ok(rel_files)
}

// TRAITS DEFINITIONS
pub trait Compiler {
    fn compile_file(&self, source_dir: &Path, project_path: &String, rel_file_path: &String, verbose:bool) -> Result<(), &'static str>;
    fn compile_project(&self, project: Project, project_path: PathBuf, working_dir: PathBuf, verbose:bool) -> Result<(), &'static str>;
    fn detect_compiler_path() -> Option<String>;
}
