use std::fs::exists;
use std::path::{Path, PathBuf};

use std::{env, io};
use std::env::args;
use std::io::{Write};

use std::process::{Command, Output};

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


fn compile_file(gcc_path: &String, source_dir: &Path, rel_file_path: &String) -> Result<(), &'static str> {

    // rel_file_path is relative to source_dir
    let output = Command::new(&gcc_path)
        .arg("-c") // Stops after compilation
        .arg(rel_file_path) // Input file
        .current_dir(source_dir.canonicalize().unwrap()) // Set working directory
        .output()
        .expect(format!(
            "(?) Failed to compile source file {}{}",
            source_dir.display(), rel_file_path.as_str()
        ).as_str());

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).map_err(|_| "Failed to write to stdout")?;
    io::stderr().write_all(&output.stderr).map_err(|_| "Failed to write to stderr")?;

    if output.status.success() {
        println!("(!) > Compiled successfully.");
        Ok(())
    } else {
        return Err("(?) > Compilation failed.");
    }
}

pub fn compile_project(project: Project, config_path:PathBuf) -> Result<(), &'static str> {
    let gcc_path = detect_gcc_path().ok_or("GCC compiler not found")?;
    println!(
        "\n(!) >> Compiling Project: {} ({}) using GCC at {}\n",
        project.name, project.path.display(), &gcc_path
    );




    let source_dir = config_path
        .canonicalize(); // Turns into abs path

    if source_dir.is_err() {
        return Err("(??) Cannot canonicalize project directory!! That probably means the directory doesnt exist");
    }

    let source_dir = source_dir
        .expect(format!("(??) Directory {}{} not found for project {}", config_path.display(), project.path.display(), project.name).as_str()) // Error handling when turning into abs path
        .parent()
        .unwrap()
        .join(&project.path);


    let files = list_files(&source_dir.canonicalize().unwrap()).map_err(|_| "Failed to list source files")?;
    
    for source_file in files {
        // PathBuf -> OsString -> String
        let source_file_str = source_file.to_string_lossy().into_owned();

        // Here is the dirty work
        println!("(!) > Compiling source file: {}", &source_file_str);
        compile_file(&gcc_path, &source_dir, &source_file_str)?;
        println!("(!!) >> Project compiled successfully");

        //TODO: Link

    }
    Ok(())
}