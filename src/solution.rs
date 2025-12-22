use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum TargetArch {
    X86,
    X64,
    ARM,
    ARM64,
}

#[derive(Deserialize)]
pub enum ProjectType {
    StaticLib,
    DynamicLib,
    Executable,
}

#[derive(Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub project_type: ProjectType,
    pub target_archs: Vec<TargetArch>,
    pub path: PathBuf,
    pub dependencies: HashMap<String, String>,
}

impl Project {
    pub fn new(
        name: &str,
        version: &str,
        project_type: ProjectType,
        target_archs: Vec<TargetArch>,
        path: PathBuf,
        dependencies: HashMap<String, String>,
    ) -> Self {
        Project {
            name: name.to_string(),
            version: version.to_string(),
            project_type,
            target_archs,
            path,
            dependencies,
        }
    }
}

#[derive(Deserialize)]
pub struct Solution {
    pub name: String,
    pub projects: Vec<Project>,
}