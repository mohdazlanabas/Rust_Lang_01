use std::fmt::{Debug, Display};

struct Cardinal;
struct BlueJay;
struct Turkey;
trait Red {}
trait Blue {}
impl Red for Cardinal{}
impl Blue for BlueJay {}
fn red<T: Red>(_: &T) -> &'static str { "red"}
fn blue<T: Blue>(_: &T) -> &'static str {"blue"}

// IMPL

struct Val { val: f64, }
struct GenVal<T> { gen_val:T,}
impl Val { fn value(&self) -> &f64 { &self.val}}
impl<T> GenVal<T> { fn value(&self) -> &T { &self.gen_val}}

// TRAITS
struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

//BOUNDS
trait HasArea{ fn area(&self) -> f64;}
impl HasArea for Rectangle {
    fn area(&self) -> f64 {self.length * self.height}
}

#[derive(Debug)]
struct Rectangle {length: f64, height:f64}
#[allow(dead_code)]
struct Triangle {length: f64, height:f64}

fn print_debug<T: Debug>(t: &T) { println!("{:?}", t);}
#[allow(dead_code)]
fn area<T: HasArea>(t: &T) -> f64 {t.area()}

// MULTIPLE BOUNDS

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: '{:?}'", t);
    println!("u: '{:?}'", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: '{:?}'", t);
    println!("u: '{:?}'",u);
}

// WHERE CLAUSES

trait PrintInOption {
    fn print_in_option(self);
}
impl<T> PrintInOption for T where
Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

// NEW TYPE IDIOM
struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}
impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {

    #![allow(unused)]
    struct S;
    struct GenericVal<T>(T);

    // IMPL

    impl GenericVal<f32> {}
    impl GenericVal<S> {}
    impl<T> GenericVal<T> {}

    let x = Val {val: 3.0};
    let y = GenVal {gen_val: 3i32};
    println!("{}, {}", x.value(), y.value());

    // TRAIT
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);

    //BOUNDS
    let rectangle = Rectangle {length: 3.0, height: 4.0};
    let _triangle = Triangle {length: 3.0, height: 4.0};
    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());

    // TEST CASE
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue is {}", blue(&blue_jay));

    // MULTIPLE BOUNDS

    let string = "words";
    let array = [1,2,3];
    let vec = vec![1,2,3];
    compare_prints(&string);
    compare_types(&array, &vec);

    // WHERE CLAUSES
    let vec = vec![1,2,3];
    vec.print_in_option();

    // NEW TYPE IDIOM

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));

    // CLOSING BRRACKETS
}