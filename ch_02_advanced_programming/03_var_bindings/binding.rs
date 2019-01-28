// variable binding refers to how a var in Rust code is bound to a type
// - pattern, mutablilty, scope, and shadow

fn main() {
    println!("***\nBindings\n*** \n");
    // Simplest variable binding
    let a = 5;

    // pattern
    let (b, c) = (1, 2);

    // Type annotation
    let x_val: i32 = 5;

    // Shadow example
    let y_val: i32 = 8;

    // Variable bindings have a scope:
    {
        println!("Val which was assigned when entering the scope : {}", y_val);
        let y_val = 12;
        println!("Value modified within scope: {}", y_val); // 12
        println!("---- \n");
    }

    println!("Value which was assigned first: {}", y_val); // 8
    println!("---- \n");

    let y_val = 42;
    println!("New value assigned: {}", y_val); // 42
    println!("---- \n");
}
