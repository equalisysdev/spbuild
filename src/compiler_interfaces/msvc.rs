use std::fs::exists;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;

use crate::solution::Project;

pub fn detect_msvc_path() -> Result<String, &'static str> {

    // Using vswhere to locate latest Visual Studio installation
    // vswhere.exe is ALWAYS installed in %ProgramFiles(x86)%\Microsoft Visual Studio\Installer
    let program_files_x86 = env::var("ProgramFiles(x86)").unwrap_or("C:\\Program Files (x86)".to_string());

    // Check for vswhere existence
    let vswhere = PathBuf::from(program_files_x86)
        .join("Microsoft Visual Studio")
        .join("Installer")
        .join("vswhere.exe");

    if !exists(&vswhere).unwrap_or(false) {
        return Err("Missing vswhere. Please install Visual Studio to continue.");
    }

    // Real stuff: getting the installation path
    let output = Command::new(vswhere)
        .args(&["-latest", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"])
        .output()
        .map_err(|_| "Failed to execute vswhere")?;

    if !output.status.success() {
        return Err("No valid Visual Studio installation found by vswhere");
    }

    let installation_path = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if installation_path.is_empty() {
        return Err("No valid Visual Studio installation found by vswhere");
    }

    let msvc_base = PathBuf::from(&installation_path)
        .join("VC")
        .join("Tools")
        .join("MSVC");

    // You'll need to find the latest MSVC version directory here
    // For now, returning the base path as a placeholder
    if exists(&msvc_base).unwrap_or(false) {
        msvc_base.to_str()
            .ok_or("Failed to convert path to string")
            .map(|s| s.to_string())
    } else {
        Err("MSVC path does not exist")
    }
}

fn compile(path: &Path) -> Result<(), &'static str> {

    Ok(())
}

fn link(files: &Vec<&Path>) -> Result<(), &'static str> {

    Ok(())
}

pub fn build_project(project: Project) -> Result<(), &'static str> {

    let paths = vec![
        Path::new("file1.o"),
        Path::new("file2.o"),
    ];

    // link(&paths);

    Ok(())
}
