fn main() {
    println!("Decimal: \n");

    // Prints the first two numbers after the decimal points
    println!("Prints the first two numbers after the decimal points \n");
    let x = 1.2345;
    println!("{}, formatted is: {:.2}", x, x);
    println!("=============");

    println!("Print the binary, hex, and octal format: \n");
    println!("B: {:b} H: {:x} O: {:0}", 10, 10, 10);
    println!("============= \n");

    println!("Shifts: \n");
    println!("Still not sure how this works...");
    println!("{ten:>ws$}", ten = 10, ws = 50);
    println!("{ten:>0ws$}", ten = 10, ws = 50);
    println!("============= \n");
}
