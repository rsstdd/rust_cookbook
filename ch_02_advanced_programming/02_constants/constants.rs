// Global variables are declared outside scopes of other fn
// Constants represent a val not a memory address
const UPPERLIMIT: i32 = 12;

fn is_big(n: i32) -> bool {
    // Access constant in the main thread
    println!("Access constant in the main thread");
    n > UPPERLIMIT
}

fn main() {
    println!("***\nConstants\n*** \n");
    println!("Global variables are declared outside scopes of other fn");

    let random_number = 15;

    println!("Threadhold is {}", UPPERLIMIT);

    // Access constant in the main thread
    println!("Access constant in the main thread");
    println!("{} is {}", random_number, if is_big(random_number) { "big" } else { "small" });

    // UPPERLIMIT = 5;
    println!("---- \n");
}
