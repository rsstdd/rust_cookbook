//! Implementing panic

fn compare_stmt(stmt: &str) {
    if stmt == "Another Book" {
        panic!("Rust Cookbook is not selected");
    }

    println!("Statement is {}", stmt);
}


fn main() {
    compare_stmt("Rust Cookbook");
    compare_stmt("Another Book");
}
