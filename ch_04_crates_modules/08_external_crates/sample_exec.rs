//! Create a sample executor of sample_lib in rust
//! created in 07_build_libs
//! Used via `rustc sample_exec.rs --extern sample_lib=libsample_lib.rlib`

extern crate sample_lib;

fn main() {
    sample_lib::pub_func();
    sample_lib::indirect_acc();
}
