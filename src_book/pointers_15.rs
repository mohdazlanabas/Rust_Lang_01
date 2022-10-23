use std::ops::Deref;
use crate::List::{Cons, Nil};
use std::rc::Rc;

//enum List{
//    Cons(i32, List),
//    Nil,}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T) ;

impl<T> MyBox<T> {
    #[allow(dead_code)]
    fn new (x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}' !", self.data);
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {

    //println!("Hello, world!");
    //// Store Data on The Heap
    //let b = Box::new(5);
    //println!("b = {}",b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // let y = &x;
    // Box<T> like a reference
    let x =5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a ={}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b ={}", Rc::strong_count(&a));
    {
    let _c = Cons(4, Rc::clone(&a));
    println!("count after creating c ={}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope ={}", Rc::strong_count(&a));
    
    let b = Box::new(5);
    println!("b = {}", b);

    


// CLOSING BRACKETS
}
