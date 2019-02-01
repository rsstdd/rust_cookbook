//! Spawning 10 threads in rust

use std::thread;

// static value NO_THREADS
static NO_THREADS: i32 = 10;

fn main() {
    let mut thread_holder = vec![];

    for i in 0..NO_THREADS {
        thread_holder.push(thread::spawn(move || {
            println!("Thread number is {}", i);
            i
        }));
    }

    println!("*********************");

    for thread_elems in thread_holder {
        println!("Thread returned {:?}", thread_elems.join().unwrap());
    }
}
