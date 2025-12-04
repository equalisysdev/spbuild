mod project;
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


use std::env;
use std::path::{Path, PathBuf};
use clap::Parser;

use crate::project::Project;
use crate::config_parser::{Config, parse_config};

use crate::compiler_interfaces::common::Compiler;


#[derive(Parser, Debug)]
#[command( version, about, long_about = None)]
struct Args {

    #[arg(short, long, help = "Path to the project configuration file")]
    project_path: String,
}

fn main() {
    //TODO: Add directory and file discovery, ~~Add config file support (See old cpp project)~~
    let args = Args::parse();
    let mut config_path = PathBuf::from(&args.project_path);

    println!("===== SPBuild Starting =====");

    if !config_path.exists() {
        eprintln!("Configuration file not found: {}", config_path.display());
        return;
    }

    if !config_path.is_file() {
        println!("> Specified path is not a file: {} <", config_path.display());
        println!("using default configuration file: spbuild.json\n");
        config_path = config_path.join("spbuild.json");
    }
    else {
        println!("Using configuration file: {}", &args.project_path);
    }

    let config = parse_config(&config_path);

    print!("Detecting platform... ");
    let current_platform = env::consts::OS;
    println!("{}", &current_platform);

    println!("\n= STARTING BUILD =\n");

    if current_platform == "windows" {
        //TODO : Call msvc functions
    }
    else if current_platform == "linux" {
        let working_dir = Path::new(&args.project_path).parent().unwrap();

        match config {
            Ok(cfg) => {
                for project in cfg.projects {
                    let compiler = compiler_interfaces::gcc::GccCompiler {
                        gcc_path: compiler_interfaces::gcc::GccCompiler::detect_compiler_path().unwrap(),
                    };

                    let res = compiler.compile_project(project, PathBuf::from(&args.project_path), PathBuf::from(&working_dir));

                    if let Err(e) = res {
                        eprintln!("Error compiling project: {}", e);
                    }
                    else {
                        println!("\n(!!) >> Project compiled successfully <<");
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse config: {}", e);
            }
        }
    }
}