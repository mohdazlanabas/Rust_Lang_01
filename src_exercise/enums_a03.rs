#![allow(dead_code)]

use crate::Status::{Poor, Rich};
use crate::Work::*;
use crate::List::*;

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},}
enum Color {
    Red = 0xf0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,}
enum List {
    Cons(u32, Box<List>),
    Nil,}
enum Status {Rich, Poor,}
enum Work{Civilian, Soldier,}
enum Number{Zero, One,Two,}

impl List {
    fn new () -> List {Nil}

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.",c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click {x,y} => { println!("clicked at x={}, y={}.", x,y);
        },}}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my tet".to_owned());
    let click = WebEvent::Click {x: 20, y:80};
    let load  = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let status = Poor;
    let work = Civilian;
    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),}
        match work {
            Civilian => println!("Civilians work!"),
            Soldier => println!("Soldier fight!"),}

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

        // CLOSING BRACKETS
    }

