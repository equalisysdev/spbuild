use std::fs::{exists, create_dir_all};
use std::path::{Path, PathBuf};

use std::{io};
use std::io::Write;

use std::process::Command;

use crate::compiler_interfaces::common::{list_files, Compiler};
use crate::project::Project;

pub struct GccCompiler {
    pub gcc_path: String,
}

impl GccCompiler {
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

impl Compiler for GccCompiler {
    fn compile_file(
        &self,
        source_dir: &Path,
        project_path: &String,
        rel_file_path: &String,
        _verbose: bool,
    ) -> Result<(), &'static str> {
        // Contract:
        // - `source_dir`: absolute path to project sources (e.g. .../example_project/alpha)
        // - `rel_file_path`: a path returned by `list_files`, currently relative to `source_dir` (e.g. ./main.c)
        // - `project_path`: CLI arg (path to spbuild.json), used to locate build root

        let build_root = Self::build_root_from_config_path(project_path)?;

        // Put outputs next to the config by project (e.g. example_project/output/alpha/...).
        let output_dir = build_root.join(format!("output/{}", source_dir.components().last().unwrap().as_os_str().to_string_lossy()));
        create_dir_all(&output_dir).map_err(|_| "Failed to create output directory")?;

        if _verbose { println!("v- using output directory {}", output_dir.display()) }

        let input_path = source_dir.join(rel_file_path);

        // Preserve subdirs if `rel_file_path` contains them.
        let rel_path = Path::new(rel_file_path);
        let out_rel = rel_path.with_extension("o");
        let output_path = output_dir.join(&out_rel);

        if let Some(parent) = output_path.parent() {
            create_dir_all(parent).map_err(|_| "Failed to create output subdirectory")?;
        }

        // Use g++ for C++ sources so the preprocessor selects the right language.
        let driver = if input_path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("cpp") || e.eq_ignore_ascii_case("cc") || e.eq_ignore_ascii_case("cxx"))
        {
            "/usr/bin/g++"
        } else {
            &self.gcc_path
        };

        if _verbose {
            println!("v- input:  {}", input_path.display());
            println!("v- output: {}", output_path.display());
        }

        let output = Command::new(driver)
            .current_dir(&build_root)
            .arg("-c")
            .arg(&input_path)
            .arg("-o")
            .arg(&output_path)
            .output()
            .map_err(|_| "Failed to execute GCC")?;

        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).map_err(|_| "Failed to write to stdout")?;
        io::stderr().write_all(&output.stderr).map_err(|_| "Failed to write to stderr")?;

        if output.status.success() {
            println!("(!) > Compiled successfully.");
            Ok(())
        } else {
            Err("(?) > Compilation failed.")
        }
    }

    fn compile_project(
        &self,
        project: Project,
        project_path: PathBuf,
        working_dir: PathBuf,
        _verbose: bool,
    ) -> Result<(), &'static str> {
        let gcc_path = GccCompiler::detect_compiler_path().ok_or("GCC compiler not found on system")?;

        println!(
            "(.) >> Compiling Project: {} ({}) using GCC at {}\n",
            project.name,
            project.path.display(),
            &gcc_path
        );

        // `working_dir` is the directory containing the config file.
        // `project.path` is a relative path inside that directory (e.g. ./alpha/)
        let source_dir = working_dir
            .join(&project.path)
            .canonicalize()
            .map_err(|_| "Project source directory not found")?;

        let files = list_files(&source_dir).map_err(|_| "Failed to list source files")?;

        for source_file in files {
            // list_files returns paths like ./main.c relative to source_dir
            let rel = source_file.to_string_lossy().into_owned();
            let project_path_str = project_path.to_string_lossy().into_owned();

            println!("(.) > Compiling source file: {}", &rel);
            self.compile_file(&source_dir, &project_path_str, &rel, _verbose)?;
        }

        Ok(())
    }

    fn detect_compiler_path() -> Option<String> {
        let gcc_path = Path::new("/usr/bin/gcc");

        if exists(gcc_path).expect("GCC path check failed") {
            gcc_path.to_str().map(|s| s.to_string())
        } else {
            None
        }
    }
}