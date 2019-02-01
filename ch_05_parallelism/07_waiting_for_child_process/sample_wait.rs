//! Waiting for a child process

use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("Status if the child process {}", _result);
    println!("Reached the end of main");
}
