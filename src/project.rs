pub enum Compilers {
    GCC,
    MSVC,
}

pub enum TargetArch {
    X86,
    X86_64,
    ARM,
    ARM64,
}

pub enum TargetType {
    StaticLib,
    DynamicLib,
    Executable,
}
pub struct Target {
    pub name: String,
    pub version: String,
    pub target: TargetType,
    pub target_arch: TargetArch,
    pub compiler: Compilers,
    pub dependencies: Vec<Target>,
}

impl Target {
    pub fn new(
        name: &str,
        version: &str,
        target: TargetType,
        target_arch: TargetArch,
        compiler: Compilers,
        dependencies: Vec<Target>,
    ) -> Self {
        Target {
            name: name.to_string(),
            version: version.to_string(),
            target,
            target_arch,
            compiler,
            dependencies,
        }
    }
}