use std::path::{Path, PathBuf};
use std::{fs, io};

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

///
/// Returns all the object files (.o) found in the given list of files.
/// DOES NOT search for files in the filesystem; use `list_files` for that.
/// # Arguments
/// * `files` - A vector of PathBuf representing files to search through.
/// * `path` - The base path to join with found object files.
/// # Returns
/// * A vector of PathBuf containing the full paths to the found object files.
///
pub fn find_object_files(files: &Vec<PathBuf>, path: &PathBuf) -> Vec<PathBuf> {
    let mut found_object_files: Vec<PathBuf> = Vec::new();

    for file in files {
        let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext.eq_ignore_ascii_case("o") {
            let obj_path = path.join(&file);
            found_object_files.push(obj_path);
        }
    }

    found_object_files
}