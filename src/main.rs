mod project;
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


use std::env;
use std::path::{Path, PathBuf};
use clap::Parser;

use crate::project::Project;
use crate::config_parser::{Config, parse_config};

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
        println!("using default configuration file: config.json\n");
        config_path = config_path.join("config.json");
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
        match config {
            Ok(cfg) => {
                for project in cfg.projects {
                    let res = compiler_interfaces::gcc::compile_project(project);

                    if res.is_err(){
                        eprintln!("Error compiling project: {:?}", res.err());
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse config: {}", e);
            }
        }
    }
}