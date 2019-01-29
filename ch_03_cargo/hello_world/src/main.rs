// Declare the external crate
extern crate regex;
use regex::Regex;

fn main() {
    // unwrap = Moves the value v out of the Option<T> if it is Some(v).
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {:?}", re.is_match("2017-02-01"));
}
