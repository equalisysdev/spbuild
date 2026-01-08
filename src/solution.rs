use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Solution {
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Deserialize, Eq, PartialEq, Clone)]
pub enum ProjectType {
    StaticLib,
    DynamicLib,
    Executable,
}

#[derive(Deserialize, Clone)]
pub enum TargetArch {
    X86,
    X64,
    ARM,
    ARM64,
}

// Implemented clone for Dependency to allow duplication when needed.
// TODO: Find a way to not use that if possible.
#[derive(Deserialize, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub optional: bool,
}

#[derive(Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub project_type: ProjectType,
    pub target_archs: Vec<TargetArch>,
    pub path: PathBuf,
    pub dependencies: Vec<Dependency>,
    pub additional_includes: Vec<PathBuf>,
}


impl Project {
    pub fn new(
        name: &str,
        version: &str,
        project_type: ProjectType,
        target_archs: Vec<TargetArch>,
        path: PathBuf,
        dependencies: Vec<Dependency>,
        additional_includes: Vec<PathBuf>,
    ) -> Self {
        Project {
            name: name.to_string(),
            version: version.to_string(),
            project_type,
            target_archs,
            path,
            dependencies,
            additional_includes,
        }
    }
}