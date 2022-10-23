use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8}

// DISPLAY

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)}}

#[derive(Debug)]
struct MinMax(i32, i32);

impl fmt:: Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)}}

#[derive(Debug)]
struct Point2D {x: f64, y: f64,}

impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)}}

// TESTCASE LIST

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        
    for (count, v) in vec.iter().enumerate() {
        if count != 0 {write!(f, ", ")?;}
        write!(f, "{}", v)?;
    }
    write!(f, "]")}}

// FORMATTING

struct City { name: &'static str,lat: f32,lon: f32,}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N'} else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else {'W'};
        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)}} 

fn main () {
    println!("Hello, World! \nI'am a Rustacean");
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("Hello {0}, this is {1} and {2}. Dear {2} amd {1}, this is {0}", "Alice", "Bob", "John");

    println!("{subject} {verb} {object}",
        object="the lazy dog", 
        subject="the quick brown fox",
        verb="jumps over");

    println!("Base 10 representation: {}", 69420);
    println!("Base 2 representation: {:b}", 69420);
    println!("Base 8 representation: {:o}", 69420);
    println!("Base 16 representation: {:x}", 69420);
    println!("Base 16 representation: {:X}", 69420);
    println!("Base Eponential representation: {:e}", 69420);

    println!("{number:>10}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0>width$}", number=1, width=10);

    println!("My name is {0}, {1} {0}", "Bond", "James");
    
    let number: f64 = 1.234;
    let width: usize = 10;
    println!("{number:>width$}");

    // DEBUG

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actors");

    println!("Now {:?} will print!", Structure(7));
    println!("Now {:?} will print", Deep(Structure(3)));

    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};
    println!("{:#?}", peter);

    // DISPLAY

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3,3);
    println!("The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range);

    let point = Point2D {x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // TESTCASE LIST

    let v = List(vec![1,2,3]);
    println!("{}", v);

    // FORMATTING

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
        ].iter() {println!("{}", *city);}

    // CLOSE BRACKETS
}

//#[derive(Debug)]
//struct Color {
//    red: u8,
//    green: u8,
//    blue: u8,/}

   //for color in [
    //Color { red: 128, green: 255, blue: 90 },
    //Color { red: 0, green: 3, blue: 254 },
    //Color { red: 0, green: 0, blue: 0 },
    //].iter() {
    //// Switch this to use {} once you've added an implementation
    //// for fmt::Display.
    //println!("{:?}", *color);}