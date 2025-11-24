mod target;
use std::env;

mod gcc {
    include!("compiler_interfaces/gcc.rs");
}

mod msvc {
    include!("compiler_interfaces/msvc.rs");
}

fn main() {
    println!("Hello, world!");

    let platform = env::consts::OS;

    //TODO: Add directory and file discovery, Add config file support (See old cpp project)
    // Maybe put that in another file ?
    // The compiler_interfaces also probably need some refactoring

    if platform== "windows" {
        //TODO : Call msvc functions
    }
    else if platform == "linux" {

    }
}