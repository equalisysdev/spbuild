use std::fs::{exists, create_dir, create_dir_all};
use std::path::{Path, PathBuf};

use std::{env, io};
use std::env::args;
use std::io::{Write};

use std::process::{Command, Output};

use crate::compiler_interfaces::common::{list_files, Compiler};
use crate::project::Project;

pub struct GccCompiler {
    pub gcc_path: String,
}

impl Compiler for GccCompiler {
    fn compile_file(&self, source_dir: &Path, project_path: &String, rel_file_path: &String) -> Result<(), &'static str> {
        // rel_file_path is relative to source_dir
        let output_path = PathBuf::new().join(project_path).canonicalize().unwrap().join("output");

        if Path::new(&project_path).exists() == false {
                create_dir_all(&project_path).map_err(|_| "Failed to create output directory")?;
        }

        let output = Command::new(&self.gcc_path)
            .arg("-c") // Stops after compilation
            .arg(rel_file_path) // Input file
            .arg("-o")
            .arg(project_path)
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

    fn compile_project(&self, project: Project, project_path: PathBuf, working_dir: PathBuf) -> Result<(), &'static str> {
        let gcc_path = GccCompiler::detect_compiler_path().ok_or("GCC compiler not found on system")?;

        println!(
            "(.) >> Compiling Project: {} ({}) using GCC at {}\n",
            project.name, project.path.display(), &gcc_path
        );

        let source_dir = &project_path.join(working_dir)
            .canonicalize()
            .unwrap()
            .join(&project.path); // Turns into abs path

        let files = list_files(&source_dir.canonicalize().unwrap()).map_err(|_| "Failed to list source files")?;

        for source_file in files {
            // PathBuf -> OsString -> String
            let source_file_str = source_file.to_string_lossy().into_owned();
            let project_path_str = project_path.to_string_lossy().into_owned();

            // Here is the dirty work
            println!("(.) > Compiling source file: {}", &source_file_str);
            self.compile_file(&source_dir, &project_path_str, &source_file_str)?;


            //TODO: Linking not implemented yet

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