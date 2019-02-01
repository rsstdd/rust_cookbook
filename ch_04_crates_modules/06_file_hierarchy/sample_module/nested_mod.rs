//! Nested module - Demonstrates a sample file structure

pub fn sample_nested_func() {
    println!("called sample_module::nested::sample_nested_func()")
}

#[allow(dead_code)]
fn nested_private_func() {
    println!("called
        sample_module::nested::nested_private_func()"
    )
}
