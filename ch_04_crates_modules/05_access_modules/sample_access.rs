//! Creates a sample module to illustrating `self` and `super`
mod sample_module;
mod cool;

fn sample_function() {
    println!("Called `sample_function` outside of the mod sample_mod");
}

fn main() {
    sample_mod::cool::indirect_call();

    {
        // Bind to cool::sample_function in the *crate* scope
        // In this case, that's the outermost scope.
        use cool::sample_function as root_sample_function;
        root_sample_function();
    }
}
