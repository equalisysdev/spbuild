use std::fs::exists;
use std::path::Path;

use std::io;
use std::io::{Write};

use std::process::Command;

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

fn compile_file(file: &String, gcc_path: &String) -> Result<(), &'static str> {

    let output = Command::new(&gcc_path)
        .arg("-c")
        .arg(file)
        .output()
        .expect("Failed to compile source file");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).map_err(|_| "Failed to write to stdout")?;
    io::stderr().write_all(&output.stderr).map_err(|_| "Failed to write to stderr")?;

    if output.status.success() {
        println!("Compiled successfully.");
        Ok(())
    } else {
        return Err("Compilation failed.");
    }
}

pub fn compile_project(project: Project) -> Result<(), &'static str> {
    let gcc_path = detect_gcc_path().ok_or("GCC compiler not found")?;
    println!(
        "=> Compiling Project: {} using GCC at {}",
        project.name, &gcc_path
    );

    let files = list_files(Path::new(&project.path)).map_err(|_| "Failed to list source files")?;
    
    for source_file in files {
                                              // PathBuf -> OsString -> String
        let source_file_str = source_file.into_os_string().into_string().unwrap();

        // Here is the dirty work
        println!("Compiling source file: {}", source_file_str);
        compile_file(&source_file_str, &gcc_path)?;

        //TODO: Link

    }
    Ok(())
}