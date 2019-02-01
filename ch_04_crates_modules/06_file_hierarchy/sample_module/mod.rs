//! Sample module - creating a file structure

mod sample_private;
pub mod nested_mod;

pub fn sample_mod_func() {
    println!("called `sample_module::sample_mod_func` \n");
}

fn private_func() {
    println!("called `sample_module::private_func()` \n");
}

pub fn indeirect_acc() {
    println!("called `sample_module::indeirect_acc()` \n");
    private_func();
}
