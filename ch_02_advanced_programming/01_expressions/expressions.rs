fn main() {
    println!("***\nExpression\n*** \n");
    let x_val = 5u32;

    // y block
    println!("y block:");
    let y_val = {
        let x_squared = x_val * x_val;
        let x_cube = x_squared * x_val;

        println!("x_cube + x_squared + x_val is y_val: {}", x_cube + x_squared + x_val);
        x_cube + x_squared + x_val
    };
    println!(" \n");


    // z block
    println!("z block");
    let z_block = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        println!("2 * x_val; => The semicolon suppresses this expression and `()` is assigned to `z`");
        2 * x_val;
    };
    println!(" \n");

    println!("---- \n");
    println!("Printing the final outcomes:");
    println!("x is {:?}", x_val);
    println!("y is {:?}", y_val);
    println!("z is {:?}", z_block);
    println!("---- \n");
}
