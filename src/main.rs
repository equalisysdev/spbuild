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
use std::path::Path;
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
    println!("Hello, world!");

    //TODO: Add directory and file discovery, ~~Add config file support (See old cpp project)~~

    let args = Args::parse();
    let config = parse_config(Path::new(&args.project_path));

    let current_platform = env::consts::OS;

    if current_platform == "windows" {
        //TODO : Call msvc functions
    }
    else if current_platform == "linux" {
        for project in config.unwrap().projects {
            let res = compiler_interfaces::gcc::compile_project(project);

            if res.is_err(){
                eprintln!("Error compiling project: {:?}", res.err());
            }
        }
    }
}