use std::{f64};

fn main() {
    println!("***\nImplements\n*** \n");

    // variable of circle data type
    println!("variable of circle data type");
    let mut circle1 = Circle {
        r: 10.0,
    };
    println!("---- \n");

    // variable of rectangle type
    let mut rect = Rectangle {
        h: 10.0,
        b: 10.0,
    };
    println!("Area of rectangle {}", rect.area());
    println!("Area of circle {}", circle1.area());
}

// user defined data type rectangle
struct Rectangle {
    h: f64,
    b: f64,
}

// user defined data type circle
struct Circle {
    r: f64,
}

// create functionality for the data types
trait HasArea {
    fn area(&self) -> f64;
}

// implement area for circle
impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14 * (self.r  * self.r)
    }
}

// implement area for rectangle
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.h * self.b
    }
}
