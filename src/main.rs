mod target;

mod gcc {
    include!("compiler_interfaces/gcc.rs");
}

fn main() {
    println!("Hello, world!");
}