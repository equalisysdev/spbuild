mod solution;
mod config_parser;

mod compiler_interfaces {
    pub mod common {
        use std::path::Path;
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
}

use std::env;
use std::fmt::format;
use std::path::{Path, PathBuf};
use clap::Parser;

use crate::helpers::console::Console;

use crate::config_parser::{parse_config};

use crate::compiler_interfaces::common::Compiler;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to the solution configuration file")]
    solution_path: String,

    #[arg(short, long, action = clap::ArgAction::SetTrue, help = "Enable verbose output")]
    verbose: bool,
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

    let config = parse_config(&config_path);

    Console::log_info("Detecting platform... ");
    let current_platform = env::consts::OS;
    Console::log_info(format!("{}!", &current_platform).as_str());

    Console::log_info("\n= STARTING BUILD =\n");

    if current_platform == "windows" {
        Console::log_fatal("Windows platform detected. MSVC support is not yet implemented.");
        Console::log_fatal("==== Aborting build ====");
        //TODO : Call msvc functions
    }
    else if current_platform == "linux" {
        // Use the directory containing the resolved config file, not the raw CLI arg.
        let working_dir = config_path
            .parent()
            .expect("Config path has no parent")
            .to_path_buf();

        match config {
            Ok(solution) => {
                for project in solution.projects {
                    let compiler = compiler_interfaces::gcc::GccCompiler {
                        gcc_path: compiler_interfaces::gcc::GccCompiler::detect_compiler_path().unwrap(),
                    };

                    let res = compiler.compile_project(
                        project,
                        config_path.clone(),
                        working_dir.clone(),
                        args.verbose,
                    );

                    if let Err(e) = res {
                        Console::log_fatal(format!("Error compiling project: {}", e).as_str());
                    } else {
                        Console::log_success("=== Project compiled successfully ===");
                    }
                }
            }
            Err(e) => {
                Console::log_fatal(format!("Failed to parse config: {}", e).as_str());
                Console::log_fatal("==== Aborting build ====");
            }
        }
    }
}