fn main() {
    println!("Hello, World!");
    fizzbuzz_to(100);

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0,1.0);
    let _pair = Pair(Box::new(1), Box::new(2));
    // pair.destroy;
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {return false;}
    lhs % rhs ==0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n,3) {
        println!("fizz");
    } else if is_divisible_by(n,5) {
        println!("buzz");
    } else {
        println!("{}",n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

// ASSOCIATED FUNCTIONS AND METHODS

struct Point {x: f64, y: f64,}

impl Point{
    fn origin() -> Point {
        Point {x: 00., y: 0.0 }
    }

    fn new(_x: f64, y:f64) -> Point {
        Point{x:y, y:y}
    }
}

struct Rectangle{p1: Point, p2:Point,}

impl Rectangle{
    fn area(&self) -> f64 {
        let Point {x:x1, y:y1} = self.p1;
        let Point {x:x2, y:y2} = self.p2;
        ((x1-x2) + (y1-y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point {x:x1, y:y1} = self.p1;
        let Point {x:x2, y:y2} = self.p2;
        2.0 * ((x1-x2).abs() + (y1-y2).abs())
    }
    fn translate(&mut self, _x: f64, y: f64) {
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    #[allow(dead_code)]
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroy Pair({}, {}", first, second);
    }
}