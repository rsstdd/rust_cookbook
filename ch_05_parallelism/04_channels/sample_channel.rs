//! Using channels to perform safe pass of data b/w threads

use std::sync::mpsc::{ Sender, Receiver };
use std::sync::mpsc;
use std::thread;

// static value NO_THREADS
static NO_THREADS: i32 = 3;

fn main() {
    // creating endpoints of the channels
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for thread_no in 0..NO_THREADS {
        // Cloning the sender
        let thread_tx = tx.clone();

        // Sending threads via the channel
        thread::spawn(move || {
            // thread sends the message to the channel
            thread_tx.send(thread_no).unwrap();
            println!("threads {} finished", thread_no);
        });
    }

    // collecting the threads
    let mut thread_holder = Vec::with_capacity(NO_THREADS as usize);
    for i in 0..NO_THREADS {
        // Get the message from channel
        thread_holder.push(rx.recv());
    }

    println!("{:?}", thread_holder);
}
