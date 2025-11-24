use std::fs::exists;
use std::path::Path;

use crate::target::Target;
fn detect_gcc_path() -> Option<String> {
    let gcc_path = Path::new("/usr/bin/gcc");

    if exists(gcc_path).expect("GCC path check failed") {
        gcc_path.to_str().map(|s| s.to_string())
    } else {
        None
    }
}

fn compile_target(target: Target) -> Result<(), &'static str> {
    let gcc_path = detect_gcc_path().ok_or("GCC compiler not found")?;
    println!(
        "Compiling target: {} using GCC at {}",
        target.name, gcc_path
    );

    // Here you would add the actual compilation logic using GCC

    Ok(())
}