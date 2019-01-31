//! To create a sample nested_mod module

mod sample_mod {
    pub mod nested_mod {
        pub fn function() {
            println!("Called `sample_mod::nested_mod::function()`")
        }

        // Still not accessable
        #[allow(dead_code)]
        fn private_function() {
            println!("Called `sample_mod::nested_mod::private_function()`")
        }
    }
}

fn main() {
    println!("***\nNested Modules\n*** \n");
    sample_mod::nested_mod::function();
    println!("---- \n");
}
