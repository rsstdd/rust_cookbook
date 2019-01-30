mod sample_mod {
    // default private visibility
    fn private_function() {
        println!("called `sample_mod::private_function()` \n");
    }

    pub fn sample_function() {
        println!("called sample_mod::sample_function() \n");
    }

    pub fn indirect_private_fn() {
        print!("called `sample_mod::indirect_private_fn()` that \n");
        private_function();
    }
}

fn sample_function() {
    print!("called the sample_function() which is not a part of mod `sample_mod` \n");
}

fn main() {
    println!("***\nModules\n*** \n");
    sample_function();
    sample_mod::sample_function();
    sample_mod::indirect_private_fn();

    // ERROR! `private_function` is private
    // sample_mod::private_function();

    println!("---- \n");
}
