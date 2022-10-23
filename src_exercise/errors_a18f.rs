#![allow(dead_code)]
#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi}
#[derive(Debug)] enum Day {Monday, Tuesday, Wednesday}

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

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
    }

fn eat(food: Food, day:Day) {
    match cookable_v2(food) {
            Some(food) => println!("Yay On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We dont get to eat on {:?}?", day),
        }
    }

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food:: Steak, Food::Sushi);
    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}