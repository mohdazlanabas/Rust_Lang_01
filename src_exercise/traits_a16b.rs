use std::ops;
// dyn
struct Sheep{}
struct Cow{}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "bahhhhhh!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

// Overloading

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

#[allow(unused_variables)]
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, rhs:Bar) -> FooBar {
        println!(">Foo.add(Bar) was called");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

fn main() {

    // Dyn
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You have randomly chosen an animal, and it says {}", animal.noise());

    // Overload
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);


}