// Prim libs
use std::io::stdin;

fn main() {
    println!("Undersanding Assignment");
    print!("########################### \n");
    print!("########################### \n \n");

    // Compiler figues out Type if not Specified
    print!("Compiler figues out Type if not Specified \n");
    let num = 10;
    println!("Num is {}", num);
    print!("========================================== \n \n");

    // 32 bit integer
    print!("32 bit integer \n");
    let age: i32 = 40;
    println!("Age is {}", age);
    print!("========================================== \n \n");

    // Max and min of a 16 bit integer
    // (Will not compile example) //
    print!("Max and min of a 8 bit integer \n");
    println!("Max i8 {}", i8::max_value());
    println!("Min i8 {}", i8::min_value());
    print!("========================================== \n \n");

    // Max and min of a 16 bit integer
    // (Will not compile example) //
    print!("Max and min of a 16 bit integer \n");
    println!("Max i16 {}", i16::max_value());
    println!("Min i16 {}", i16::min_value());
    print!("========================================== \n \n");

    // Max and min of a 32 bit integer
    // (Will not compile example) //
    print!("Max and min of a 32 bit integer \n");
    println!("Max i32 {}", i32::max_value());
    println!("Min i32 {}", i32::min_value());
    print!("========================================== \n \n");

    // Max and min of a 32 bit integer
    // (Will not compile example) //
    print!("Max and min of a 64 bit integer \n");
    println!("Max i64 {}", i64::max_value());
    println!("Min i64 {}", i64::min_value());
    print!("========================================== \n \n");

    // Multiple variable assignment
    // Printing: Specify order of printed variables
    print!("Printing: Specify order of printed variables \n");
    let(f_name, l_name) = ("viki", "dangers");
    println!("First name {0} and last name {1}", f_name, l_name);
    print!("========================================== \n \n");
}
