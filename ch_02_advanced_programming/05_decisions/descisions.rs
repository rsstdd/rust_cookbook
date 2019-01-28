// Rust Does not provider automatic Type Casting
// - pattern, mutablilty, scope, and shadow

use std::{i32};

fn main() {
    println!("***\nCondition\n*** \n");
    let age: i32 = 10;

    // If/else
    if age <= 18 {
        println!("Go to School");
        println!("---- \n");
    } else if (age > 18) && (age <= 28) {
        println!("Do something w/ your life.");
        println!("---- \n");
    }

    let can_vote = if age >= 18 { true } else { false };
    println!("Can vote: {}", can_vote);
    println!("---- \n");
}
