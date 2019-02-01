// ! Create a sample library in Rust
//! Create lib with `rustc crate-type=lib name_of_lib.ra`

pub fn pub_func() {
    println!("Called sample_lib::pub_func()");
}

fn priv_func() {
    println!("Called sample_lib `priv_func()`");
}

pub fn indirect_acc() {
    println!("Called priv_func() from sample_lib indirect_acc()");
    priv_func();
}
