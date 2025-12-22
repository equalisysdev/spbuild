use std::fs::{exists, create_dir_all};
use std::path::{Path, PathBuf};

use std::{io};
use std::fmt::format;
use std::io::Write;

use std::process::Command;

use crate::compiler_interfaces::common::{list_files, Compiler};
use crate::helpers::console::Console;
use crate::solution::Project;

pub struct GccCompiler {
    pub gcc_path: String,
}

impl Compiler for GccCompiler {
    fn compile_file(
        &self,
        project_source: &Path,
        solution_root: &String,
        rel_file_path: &String,
        _verbose: bool,
    ) -> Result<(), &'static str> {
        // Contract:
        // - `rel_file_path`: The path to the file to compile. currently relative to `project_source` (e.g. ./main.c)
        // Check the README for other naming details.

        let build_root = Self::build_root_from_config_path(solution_root)?;

        // Put outputs next to the config by project (e.g. example_solution/output/alpha/...).
        let output_dir = build_root.join(format!("output/{}", project_source.components().last().unwrap().as_os_str().to_string_lossy()));
        create_dir_all(&output_dir).map_err(|_| "Failed to create output directory")?;

        Console::log_verbose(&format!("v- using output directory {}", output_dir.display()), _verbose);

        let input_path = project_source.join(rel_file_path);

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

        Console::log_verbose(&format!("input:  {}", input_path.display()), _verbose);
        Console::log_verbose(&format!("output: {}", output_path.display()), _verbose);


        let output = Command::new(driver)
            .current_dir(&build_root)
            .arg("-c")
            .arg(&input_path)
            .arg("-o")
            .arg(&output_path)
            .output()
            .map_err(|_| "Failed to execute GCC")?;

        Console::log_info(&format!("status: {}", output.status));

        io::stdout().write_all(&output.stdout).map_err(|_| "Failed to write to stdout")?;
        io::stderr().write_all(&output.stderr).map_err(|_| "Failed to write to stderr")?;

        if output.status.success() {
            Console::log_info(&format!(">> {} compiled successfully!", rel_file_path));
            Ok(())
        } else {
            Err("Compilation failed.")
        }
    }

    fn compile_project(
        &self,
        project: Project,
        project_path: PathBuf,
        solution_root: PathBuf,
        _verbose: bool,
    ) -> Result<(), &'static str> {
        let gcc_path = GccCompiler::detect_compiler_path().ok_or("GCC compiler not found on system")?;

        Console::log_info(&format!(
            "Compiling Project: {} version {} ({}) using GCC at {}\n",
            project.name,
            project.version,
            project.path.display(),
            &gcc_path
        ));

        // `working_dir` is the directory containing the config file.
        // `project.path` is a relative path inside that directory (e.g. ./alpha/)
        let source_dir = solution_root
            .join(&project.path)
            .canonicalize()
            .map_err(|_| "Project source directory not found")?;

        let files = list_files(&source_dir).map_err(|_| "Failed to list source files")?;

        for source_file in files {
            // list_files returns paths like ./main.c relative to source_dir
            let rel = source_file.to_string_lossy().into_owned();
            let project_path_str = project_path.to_string_lossy().into_owned();

            Console::log_info(&format!("Compiling source file: {}", &rel));
            self.compile_file(&source_dir, &project_path_str, &rel, _verbose)?;
        }

        Ok(())
    }

    fn link_project(&self, project: Project, project_path: PathBuf, verbose: bool) -> Result<(), &'static str> {
        // Contract:
        // - `project`: Project object
        // - `project_path`: absolute path to directory containing config file (e.g. .../example_solution/)

        let build_root = Self::build_root_from_config_path(&project_path.to_string_lossy())?;

        let working_dir = &build_root.join(&project_path).join("output").join(&project.path);

        let files = list_files(working_dir).map_err(|_| "Failed to list object files")?;
        let mut object_files: Vec<String> = Vec::new();
        for file in files {
            let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext.eq_ignore_ascii_case("o") {
                let obj_path = working_dir.join(&file);
                object_files.push(obj_path.to_string_lossy().into_owned());
            }
        }

        // For project `alpha`, output executable is at `<project_root>/output/alpha/alpha.exe`.
        let output_executable = build_root.join(format!("output/{}/{}.exe", &project.path.display(), &project.name));
        Console::log_info(&format!("Linking executable: {}", output_executable.display()));

        let mut command = Command::new(&self.gcc_path);
        command.current_dir(&build_root);
        for obj in &object_files {
            command.arg(obj);
        }
        command.arg("-o").arg(&output_executable);

        Console::log_verbose(&format!("Linking command: {:?}", command), verbose);

        let output = command
            .output()
            .map_err(|_| "Failed to execute GCC for linking")?;

        println!("status: {}", output.status);

        io::stdout().write_all(&output.stdout).map_err(|_| "Failed to write to stdout")?;
        io::stderr().write_all(&output.stderr).map_err(|_| "Failed to write to stderr")?;
        if output.status.success() {
            Console::log_success("Linked successfully.");
            Ok(())
        } else {
            Err("Linking failed.")
        }
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