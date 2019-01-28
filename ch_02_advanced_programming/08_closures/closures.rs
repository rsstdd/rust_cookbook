use std::{i32};

fn main() {
    println!("***\nClosurees\n*** \n");
    // Define a closure
    let sum_num = | x:i32, y:i32 | x + y;
    println!("7 + 8 is {:?}", sum_num(7, 8));
    println!("---- \n");

    // Second Closure
    let num_ten = 10;
    let add_ten = | x:i32 | x + num_ten;
    println!("x + 10 is {:?}", add_ten(10));
    println!("---- \n");
}
