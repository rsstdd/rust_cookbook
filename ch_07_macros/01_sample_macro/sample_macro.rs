//! First macro

// Syntax:
// macro_rules! macro_name {...}

macro_rules! Welcome_RustBook {
    () => (
        println!("Welcome to the Rust Cookbook!", );
    )
}

fn main() {
    Welcome_RustBook!()
}
