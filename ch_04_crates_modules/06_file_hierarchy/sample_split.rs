//! Demonstrates a sample file structure
mod sample_module;

fn sample_func() {
    print!("Called the sample_func() \n");
}

fn main() {
    sample_module::sample_mod_func();
    sample_func();
    sample_module::indeirect_acc();
    sample_module::nested_mod::sample_nested_func();
}
