#[allow(non_snake_case)]

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremails@example.com");

    let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    user1.email = String::from("anotheremails@example.com");

    let x = 1;
    let pointer01 = &x;
    println!("x: {}; address: {:p} ", x, pointer01);

    let _black = Color(0,0,0);
    let _origin = Point(0,0,0);
    let _subject = AlwaysEqual;

    // let height1 = 50;
    // let width1 = 30;
    // println!("The area of the rectangle is {} square pixels.", area(width1, height1) );

    // let rect1 = (30,50);
    // println!("The area of the rectangle is {} square pixels.", area(rect1)

    let rect1 = Rectangle { width: 30, height: 50,};
    println!("The area of the rectangle is {} square pixels.", area(&rect1)
    );
    println!("rect1 is {:?}", rect1);

    let rect2 = Rectangle2 {
        width2: 40,
        height2: 40,
    };
    println!("The area of the rectangle2 is {} square pixels.", rect2.area2());

    let rectA = Rectangle2 { width2: 20, height2: 20 };
    let rectB = Rectangle2 { width2: 10, height2: 10 };
    let rectC = Rectangle2 { width2: 30, height2: 30 };
    println!("Can rectA hold rectB {}", rectA.can_hold(&rectB));
    println!("Can rectA hold rectC {}", rectA.can_hold(&rectC));
    }

// fn area(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }  
// fn area(width: u32, height: u32) -> u32{width * height}
fn area(rectangle: &Rectangle) -> u32 { 
    rectangle.width * rectangle.height
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;
#[derive(Debug)]
struct Rectangle { width: u32, height: u32}
#[derive(Debug)]
struct Rectangle2 { width2: i32, height2: i32}

impl Rectangle2 {
    fn area2(&self) -> i32 {
        self.width2 * self.height2
    }
    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width2 > other.width2 && self.height2 > other.height2
    }
}

struct User {
    active: bool,
    username: String, // %str 
    email: String, // &str
    sign_in_count: u64,
}

fn _build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}