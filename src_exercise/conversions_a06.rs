use std::convert::{From, TryFrom, TryInto};
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
struct Number { value: i32,}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

struct Circle{radius: i32}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value: item}
        }
    }

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let _int = 5;
    let num = Number::from(30);
    println!("My number is {:?}", num);

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    let circle = Circle {radius:6};
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squ,.ared * x;
        x_cube + x_squared + x};

        let z = {2 * x;};

println!("x is {:?}", x);
println!("y is {:?}", y);
println!("z is {:?}", z);

}