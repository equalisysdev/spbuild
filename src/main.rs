mod solution;
mod config_parser;

mod compiler_interfaces {
    pub mod common {
        include!("compiler_interfaces/common.rs");
    }
    pub mod msvc {
        include!("compiler_interfaces/msvc.rs");
    }
    pub mod gcc {
        include!("compiler_interfaces/gcc.rs");
    }
}

mod helpers {
    pub mod console {
        include!("helpers/console.rs");
    }

    pub mod version_tools{
        include!("helpers/version_tools.rs");
    }

    pub mod file_tools {
        include!("helpers/file_tools.rs");
    }
}

pub mod dependency_manager {
    pub mod local_resolve {
        include!("dependency_manager/dependency_resolver.rs");
    }
}

use std::env;
use std::path::{PathBuf};
use clap::Parser;

use crate::helpers::console::Console;

use crate::config_parser::{parse_config};

use crate::compiler_interfaces::common::Compiler;
use crate::dependency_manager::local_resolve::{has_circular_dependency, resolve_project_build_inputs};
use crate::solution::Solution;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to the solution configuration file")]
    solution_path: String,

    #[arg(short, long, action = clap::ArgAction::SetTrue, help = "Enable verbose output")]
    verbose: bool,
}


//noinspection D
fn linux_build(args: Args, config_path: PathBuf, solution: Solution) {
    let working_dir = config_path
        .parent()
        .expect("Config path has no parent")
        .to_path_buf();

    Console::log_success(format!("Successfully parsed solution: {}", solution.name).as_str());

    // Track what we've already compiled to avoid rebuilding the same dependency multiple times.
    let mut compiled_projects: Vec<String> = Vec::new();

    for project in &solution.projects {
        let compiler = compiler_interfaces::gcc::GccCompiler {
            gcc_path: compiler_interfaces::gcc::GccCompiler::detect_compiler_path().unwrap(),
            gpp_path: compiler_interfaces::gcc::GccCompiler::detect_gpp_path().unwrap(),
        };

        // Resolve dependencies and include dirs.
        let inputs = match resolve_project_build_inputs(project, &solution, &working_dir, args.verbose) {
            Ok(v) => v,
            Err(e) => {
                Console::log_fatal(format!("Error resolving dependencies: {}", e).as_str());
                return;
            }
        };

        // Build local deps first.
        for dep in &inputs.local_deps_in_order {
            if compiled_projects.iter().any(|n| n == &dep.name) {
                continue;
            }

            let res = compiler.compile_project(
                dep,
                &solution,
                &working_dir,
                Vec::new(),
                args.verbose,
            );

            if let Err(e) = res {
                Console::log_fatal(format!("Error compiling dependency {}: {}", dep.name, e).as_str());
                return;
            }

            let res = compiler.link_project(
                dep,
                &solution,
                &working_dir,
                Vec::new(),
                args.verbose,
            );

            if let Err(e) = res {
                Console::log_fatal(format!("Error linking dependency {}: {}\n", dep.name, e).as_str());
                return;
            }

            compiled_projects.push(dep.name.clone());
        }

        // Compile current project with resolved include dirs.
        let res = compiler.compile_project(
            &project,
            &solution,
            &working_dir,
            inputs.include_dirs.clone(),
            args.verbose,
        );

        if let Err(e) = res {
            Console::log_fatal(format!("Error compiling project: {}", e).as_str());
            return;
        } else {
            Console::log_success("=== Project compiled successfully ===");
        }

        // Link current project.
        let mut link_inputs = inputs.dep_output_dirs.clone();
        // Keep the project's include dirs around as well in case the linker needs them later (e.g. for -L/-l).
        // For now, gcc.rs interprets these as directories to scan for `.o` files.
        link_inputs.extend(inputs.include_dirs.clone());

        let res = compiler.link_project(
            &project,
            &solution,
            &working_dir,
            link_inputs,
            args.verbose,
        );

        if let Err(e) = res {
            Console::log_fatal(format!("Error linking project: {}\n", e).as_str());
            return;
        } else {
            Console::log_success("=== Project linked successfully ===");
        }

        compiled_projects.push(project.name.clone());
    }
}


fn main() {
    let args = Args::parse();
    let mut config_path = PathBuf::from(&args.solution_path);

    Console::log_info("===== SPBuild Starting =====");

    if !config_path.exists() {
        Console::log_fatal(format!("Configuration file not found: {}", config_path.display()).as_str());
        return;
    }

    if !config_path.is_file() {
        Console::log_warning(format!("Specified path is not a file: {}", config_path.display()).as_str());
        Console::log_warning("using default configuration file: spbuild.json\n");
        config_path = config_path.join("spbuild.json");
    }
    else {
        Console::log_info(format!("Using solution configuration file: {}", &args.solution_path).as_str());
    }
    let config = parse_config(&config_path).map_err(|e| {
        Console::log_fatal(format!("Failed to parse config: {}", e).as_str());
        Console::log_fatal("==== Aborting build ====");
    }).unwrap();


    Console::log_info("Detecting platform... ");
    let current_platform = env::consts::OS;
    Console::log_info(format!("{}!", &current_platform).as_str());

    Console::log_info("\n= STARTING BUILD =\n");

    for project in &config.projects {
        if has_circular_dependency(&project, &config, &mut Vec::new()) {
            Console::log_fatal(format!("Circular dependency detected in project: {}", project.name).as_str());
            Console::log_fatal("==== Aborting build ====");
            return;
        }
    }

    if current_platform == "windows" {
        Console::log_fatal("Windows platform detected. MSVC support is not yet implemented.");
        Console::log_fatal("==== Aborting build ====");
        //TODO : Call msvc functions
    }
    else if current_platform == "linux" {
        linux_build(args, config_path, config);
    }
}