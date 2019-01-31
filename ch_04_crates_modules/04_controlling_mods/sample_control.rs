//! Sample module that illustrates 'use'

// Bind the `deeply::nested::function` path to `other function`
use deeply::nested::sample_function as other_function;

// Define a nested:
mod deeply {
    pub mod nested {
        pub fn sample_function() {
            println!("called `deeply::nested::sample_function()`");
        }
    }
}

fn sample_function() {
    println!("called `function()`");
}

fn main() {
    println!("***\nSample sample_control\n*** \n");
    // Easier access to deeply::nested::sample_function as sample_function()
    println!("About to call other_function in main \n");
    other_function();

    {
        use deeply::nested::sample_function;
        println!("About to call the fn from the block \n");
        sample_function();
        // Use bindings have a local scope. In this case, the shadowing of function()
        // is only in this block.
        println!("leaving the block \n");
    }

    sample_function();
    println!("---- \n");
}
