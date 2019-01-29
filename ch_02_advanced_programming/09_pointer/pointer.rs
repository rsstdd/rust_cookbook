use std::{i32};

fn main() {
    let vec1 = vec![1, 2, 3];

    // Err if you do this to any non-prim val:
    let vec2 = vec1;
    println!("vec1[0]: {:?}", vec1[0]);

    let prim_val = 1;
    let prim_val_2 = prim_val;
    println!("Primitive value: {}", prim_val); // Ok

    // Passing ownership top the function println!
    println!("Sum of vects: {}", sum_vects(&vec1));

    // Able to pass non-prim data type
    println!("vector 1 {:?}", vec1);
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    // apply a closure and iterator
    // An iterator method that applies a function, producing a single, final value.
    // fold() takes two arguments: an initial value, and a closure with two
    // arguments: an 'accumulator', and an element. The closure returns the
    // value that the accumulator should have for the next iteration.
    // This operation is sometimes called 'reduce' or 'inject'.
    let sum = v1.iter().fold(0, | mut sum, &x | { sum += x; sum });
    return sum;
}
