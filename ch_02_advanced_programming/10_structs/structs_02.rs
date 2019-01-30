#![allow(dead_code)]
#![allow(unused_variables)]
// Single threaded reference counting pointer. "Reference Counted"
use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Name is {}", self.name)
    }
}

fn rc_demo() {
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));

    {
        let person = Person::new(name.clone());
        person.greet();
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    }

    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    println!("name = {}", name);
}

fn main() {
    println!("***\nStructs - 02\n*** \n");
    rc_demo();
    println!("---- \n");
}
