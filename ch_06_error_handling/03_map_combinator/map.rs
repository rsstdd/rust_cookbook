//! Implementing map

#![allow(dead_code)]
#[derive(Debug)] enum Food { Apple, Carrot, Potato }
#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// Maps an Option<T> to Option<U> by applying a fn to a contained value
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f)) // Changes Type
        .map(|Peeled(food)| Chopped(food))
        .map(|Chopped(food)| Cooked(food))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some (food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = Some(Food::Potato);

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = cook(chop(peel(potato)));

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}
