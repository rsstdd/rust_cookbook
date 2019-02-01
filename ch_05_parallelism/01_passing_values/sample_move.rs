//! Passing values to a thread in rust
use std::thread;

fn main() {
    let x = 1;
    let handle = thread::spawn(move || { (x) });
    // join waits for the associated thread to finish
    // unwrap moves the value v out of the Option<T> if it is Some(v).
    println!("{:?}", handle.join().unwrap());
}
