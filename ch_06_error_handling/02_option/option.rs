
//! Implementing Option

// Some:
// None
// No value
// Some(T)
// Some value T
//
// enum Opiton<T> {
//  None,
//  Some(T),
// }

fn compare_stmt_match(input: Option<&str>) {
    match input {
        Some("Rust Cookbook") => println!("Rust cookbook was selected"),
        Some(_inner) => println!("Rust Cookbook not selected"),
        None => println!("No input provided"),
    }
}

fn compare_stmt_unwrap(input: Option<&str>) {
    let inside_val = input.unwrap(); // If Some, get that value
    if inside_val == "Another Book" { panic!("Rust Cookbook is not selected"); }
    println!("I love {}s", inside_val);
}

fn main() {
    let desired_book = Some("Rust Cookbook");
    let another_book = Some("Another Book");
    let empty_value = None;

    compare_stmt_match(desired_book);
    compare_stmt_match(another_book);
    compare_stmt_match(empty_value);

    println!("***");

    let rand_book = Some("Random Book");
    let no_val = None;

    compare_stmt_unwrap(rand_book);
    compare_stmt_unwrap(no_val);
}
