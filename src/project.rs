use serde::Deserialize;

#[derive(Deserialize)]
pub enum Compilers {
    GCC,
    MSVC,
}

#[derive(Deserialize)]
pub enum TargetArch {
    X86,
    X86_64,
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
pub struct Project {
    pub name: String,
    pub version: String,
    pub target: ProjectType,
    pub target_arch: TargetArch,
    pub rel_path: String,
    pub compiler: Compilers,
    pub dependencies: Vec<String>,
}

impl Project {
    pub fn new(
        name: &str,
        version: &str,
        target: ProjectType,
        target_arch: TargetArch,
        rel_path: String,
        compilers: Compilers,
        dependencies: Vec<String>,
    ) -> Self {
        Project {
            name: name.to_string(),
            version: version.to_string(),
            target,
            target_arch,
            rel_path,
            compiler: compilers,
            dependencies,
        }
    }
}