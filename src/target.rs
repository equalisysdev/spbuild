use std::str::FromStr;
use serde::Deserialize;
use strum_macros::EnumString;

#[derive(Deserialize, Clone, PartialEq, EnumString, Debug)]
pub enum Architecture {
    #[strum(serialize = "x86")]
    X86,
    #[strum(serialize = "x86_64")]
    X64,
    #[strum(serialize = "arm")]
    ARM,
    #[strum(serialize = "aarch64")]
    ARM64,
    #[strum(serialize = "riscv64")]
    RISCV64,
}

//TODO: These may not be the right platforms.. Maybe we'll have to make this more precise
#[derive(Deserialize, Debug, Clone, PartialEq, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    #[strum(serialize = "win")]
    Win,
    #[strum(serialize = "linux")]
    Linux,
    //TODO: Enable MacOS support later
    //#[strum(serialize = "macos")]
    //MacOS,
}

pub struct Target {
    arch: Architecture,
    platform: Platform,
}

impl Architecture {
    pub fn to_string(&self) -> String {
        match self {
            Architecture::X86 => "x86".to_string(),
            Architecture::X64 => "x86_64".to_string(),
            Architecture::ARM => "arm".to_string(),
            Architecture::ARM64 => "aarch64".to_string(),
            Architecture::RISCV64 => "riscv64".to_string(),
        }
    }

    pub fn new(arch_str: &str) -> Result<Self, String> {
        Architecture::from_str(arch_str).map_err(|_| format!("Unsupported architecture: {}", arch_str))
    }
}

impl Platform {
    pub fn to_string(&self) -> String {
        match self {
            Platform::Win => "windows".to_string(),
            Platform::Linux => "linux".to_string(),
            //Platform::MacOS => "macos".to_string(),
        }
    }

    pub fn new(arch_str: &str) -> Result<Self, String> {
        Platform::from_str(arch_str).map_err(|_| format!("Unsupported platform: {}", arch_str))
    }

    pub fn to_gcc_target_platform(&self) -> String {
        match self {
            Platform::Win => "w64-mingw32".to_string(),
            Platform::Linux => "linux-gnu".to_string(),
            //Platform::MacOS => "apple-darwin".to_string(),
        }
    }
}

impl Target {
    pub fn new(arch: Architecture, platform: Platform) -> Self {
        Self { arch, platform }
    }
}