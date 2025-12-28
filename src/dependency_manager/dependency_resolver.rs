use std::path::PathBuf;
use clap::builder::Str;
use dict::{Dict, DictEntry,DictIface};

use crate::solution::{Project, Solution};
use crate::Console;


///
/// Finds and prints local dependencies of a given project within the solution.
/// # Arguments
/// * `project` - The project whose dependencies are to be resolved.
/// * `solution` - The solution containing all projects.
/// # Returns
/// * A String, PathBuf dictionary mapping dependency names to their local paths.
///
pub fn find_local_dependencies(project: &Project, solution: &Solution) -> Dict<String>{

    let mut dep_paths: Dict<String> = Dict::<String>::new();

    for dep in &project.dependencies {

        // Checks each project for a matching name and version
        solution.projects.iter().for_each(|p| {
            if p.name.eq(&dep.name) && p.version.eq(&dep.version) {
                Console::log_info(format!(
                    "Found local dependency: {} version {} at path {}",
                    p.name,
                    p.version,
                    p.path.display()
                ).as_str());

                dep_paths.add(dep.name.clone(), p.path.to_string_lossy().into_owned());
            }
        });
    }

    dep_paths
}