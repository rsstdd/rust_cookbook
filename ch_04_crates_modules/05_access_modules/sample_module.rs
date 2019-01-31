//! Sample mod
mod sample_mod {
    fn sample_function() {
        println!("called `sample_mod::sample_function()`");
    }

    pub mod cool {
        pub fn sample_function() {
            println!("called `sample_mod::cool::sample_function()`");
        }

        pub fn indirect_call() {
            println!("called `sample_mod::cool::indirect_call()` that \n");
            // self resolves the path relative to the current module. self can only be used as the
            // first segment, without a preceding ::.
            self::sample_function(); // self == the current module scope
            self::sample_function(); // access another mod inside of sample_mod
            // super in a path resolves to the parent module. It may only be used in leading
            // segments of the path, possibly after an initial self segment.
            super::sample_function(); // `super` == parent scope
        }
    }
}
