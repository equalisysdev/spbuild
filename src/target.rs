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
