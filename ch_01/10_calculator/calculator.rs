use std::io;
use std::{i32};

fn main() {
    println!("***\nCalculator\n*** \n");

    // Request for entering num 1
    println!("Request for entering num 1");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    // Request for entering num 2
    println!("Request for entering num 2");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    // Converting a string to an int
    println!("Converting a string to an int");
    let aint: i32 = input1.trim().parse().ok().expect("Program only processes nums");
    let bint: i32 = input2.trim().parse().ok().expect("Program only processes nums");
    println!("---- \n");

    // Output of basic ops
    println!("----");
    println!("sum is : {}", aint + bint);
    println!("difference {}", aint - bint);
    println!("multiply is {}", aint * bint);
    println!("divide is {}", aint / bint);
    println!("mod is {}", aint % bint);
    println!("---- \n");
}
