use std::{i8};

fn main() {
    println!("***\nTuples\n*** \n");
    // A tuple is a unique data structure
    // Tuples are constructed by () and each tuple has a type
    // signature such as T1, T2, T3, ..., where T1..3 are
    // the types of its member values

    println!("Declaring a Tuple");
    let rand_tuple = ("Rust", 2017);
    let rand_tuple2 : (&str, i8) = ("Viki", 7);
    println!("One: {:?} and Two: {:?}", rand_tuple, rand_tuple2);
    println!("Name: {:?}; Lucky Number: {:?}", rand_tuple2.0, rand_tuple2.1);
    println!("---- \n");
}
