fn drink(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic="abort"){println!("THis is not your party. Run!!");}
        else {println!("Spit it out!!!");}
    }
    else {println!("Some refreshing {} is all I need.", beverage);}
}

fn main() {
    drink("water");
    drink("lemonade");
}