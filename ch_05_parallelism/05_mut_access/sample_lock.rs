//! Safe mutable access across threads for preventing data races

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define the main fn and declare the data variable
// with an Arc type data w/ mutex
fn main() {
    // Decalring Arc type
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    // Creating 3 threads and implementing lock
    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += 1;
            println!("Thread id: {:?}", i);
            println!("Data value: {:?}", data[0]);
            println!("\n")
        });
    }

    thread::sleep(Duration::from_millis(10));
}
