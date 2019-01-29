use std::{f64};

fn main() {
    println!("***\nStructs\n*** \n");

    // Create a struct variable
    println!("Create a struct variable");
    let circle1 = Circle {
        x: 10.0,
        radius: 10.0
    };
    println!("---- \n");


    // Print radius and variable x
    println!("Print radius and variable x");
    println!("x: {}, radius: {}", circle1.x, circle1.radius);
    println!("x: {}", circle1.get_x());
    println!("---- \n");
}

// Define your custom user type
struct Circle {
    x: f64,
    radius: f64,
}

// recommended way of creating structs
impl Circle {
    // pub makes this fn public - accessable outside of { } scope
    pub fn get_x(&self) -> f64 {
        self.x
    }
}
