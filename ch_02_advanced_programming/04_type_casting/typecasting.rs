// Rust Does not provider automatic Type Casting
// - pattern, mutablilty, scope, and shadow

use std::{i32, f32};

fn main() {
    println!("***\nType Casting\n*** \n");
    let (true_pos, true_neg, false_pos, false_neg) = (100, 50, 10, 5);

    // define a total closure
    println!("define a total closure");
    let total = true_pos + true_neg + false_pos + false_neg;
    println!("The total predictions {}", total);
    println!("---- \n");

    // Calculating the accuracy of the model
    println!(
        "Calculating the accuracy of the model {:.2}",
        percentage(accuracy(true_pos, true_neg, total))
    );

    println!("---- \n");
}

// Measures the overall performance of the model
fn accuracy(tp:i32, tn:i32, total:i32) -> f32 {
    // If !semi-colon => val last statement
    // ** No automatic typecasting in Rust **
    // "as" is most commonly used to turn primitive types into
    // other primitive types, but it has other uses that
    // include turning pointers into addresses, addresses
    // into pointers, and pointers into other pointers.
    (tp as f32 + tn as f32) / (total as f32)
}

// Convert to %
fn percentage(value: f32) -> f32 {
    value * 100.00
}
