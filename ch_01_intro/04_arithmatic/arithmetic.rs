fn main() {
    println!("\n");
    println!("Arithmetic Operations \n");

    println!("5 + 4 = {}", 5 + 4);
    println!("----------------- \n");
    println!("5 - 4 = {}", 5 - 4);
    println!("----------------- \n");
    println!("5 * 4 = {}", 5 * 4);
    println!("----------------- \n");
    println!("5 / 4 = {:2}", 5 / 4);
    println!("----------------- \n");
    println!("5 % 4 = {}", 5 % 4);
    println!("----------------- \n");
    println!("\n");

    println!("Assigning data types and mathematical operations \n");
    let neg_4 = -4i32;
    println!("abs(-4) = {}", neg_4.abs());
    println!("neg_4^2 = {}", neg_4.pow(2));
    println!("round(1.2345) = {}", 1.2345f64.round());
    println!("ceil(1.2345) = {}", 1.2345f64.ceil());
    println!("sin 3.14 = {}", 3.14f64.sin());
    println!("----------------- \n");
    println!("\n");
}
