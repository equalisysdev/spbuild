use std::path::{Path, PathBuf};

use crate::solution::{Dependency, Project, Solution};

// TRAITS DEFINITIONS
pub trait Compiler {
    fn compile_file(&self, abs_infile_path: &PathBuf, abs_output_path: &PathBuf, additional_includes: &Vec<PathBuf>, verbose:bool) -> Result<(), &'static str>;
    fn compile_project(&self, project: &Project, solution: &Solution, solution_root: &PathBuf, additional_include_directories: Vec<PathBuf>, verbose:bool) -> Result<(), &'static str>;
    fn link_project(&self, project: &Project, solution: &Solution, project_path: &PathBuf, includes_paths: Vec<PathBuf>, verbose: bool)  -> Result<(), &'static str>;
    fn detect_compiler_path() -> Option<String>;

    fn build_root_from_config_path(project_path: &str) -> Result<PathBuf, &'static str> {
        // `project_path` is the path passed from CLI (currently the config file path).
        // Canonicalize so output paths are absolute and independent of current_dir.
        let cfg = Path::new(project_path)
            .canonicalize()
            .map_err(|_| "Invalid project path")?;

        cfg.parent()
            .ok_or("Invalid project path")
            .map(|p| p.to_path_buf())
    }
}
