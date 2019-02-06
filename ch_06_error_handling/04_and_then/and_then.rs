//! and_then
#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable(food: Food) -> Option<Food> {
    // Returns None if the option is None, otherwise calls f with the wrapped value and returns the result.
    // Some languages call this operation flatmap.
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food: Food, day: Day) {
    match cookable(food) {
        Some(food) => println!("On {:?} we get to eat {:?}", day, food),
        None => println!("Oh, no! We don't get to eat on {:?}", day),
    }
}

fn main() {
    let (cordon_blue, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
    let (mon, tue, wed) = (Day::Monday, Day::Tuesday, Day::Wednesday);

    eat(cordon_blue, mon);
    eat(steak, tue);
    eat(sushi, wed);
}
