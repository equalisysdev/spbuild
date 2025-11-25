use std::fs::{self, exists, DirEntry, read_dir};
use std::path::Path;
use crate::compiler_interfaces::common::list_files;
use crate::project::Project;

pub fn detect_gcc_path() -> Option<String> {
    let gcc_path = Path::new("/usr/bin/gcc");

    if exists(gcc_path).expect("GCC path check failed") {
        gcc_path.to_str().map(|s| s.to_string())
    } else {
        None
    }
}

pub fn compile_project(project: Project) -> Result<(), &'static str> {
    let gcc_path = detect_gcc_path().ok_or("GCC compiler not found")?;
    println!(
        "Compiling Project: {} using GCC at {}",
        project.name, gcc_path
    );
    
    for source_file in list_files(project.rel_path.as_str()).unwrap() {
                                              // PathBuf -> OsString -> String
        println!("Compiling source file: {}", source_file.into_os_string().into_string().unwrap());
        // Here you would normally invoke the GCC compiler with appropriate arguments
    }
    Ok(())
}