fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {

    let x = 5u32;
    let y =x;
    println!("x is {}, and y is {}", x,y);

    let a = Box::new(5i32);
    let b =a;
    destroy_box(b);

    // MUTABILITY

    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box;
    println!("mutable_box now contains {}", mutable_box);
    *mutable_box = 4;

    #[derive(Debug)]
    struct Person {
        name: String,
        age:Box<u8>,
    }

    // PARTIAL

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person {name, ref age} = person;
    println!("The persons age is {}", age);
    println!("The person name is {}", name);
    println!("The person age from person struct is {}", person.age);

    // BORROWING

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        let _ref_to_i32 = &boxed_i32;
        // eat_box_i32(boxed_i32);
        borrow_i32(_ref_to_i32);
    }
    eat_box_i32(boxed_i32);
    // CLOSING BRACKETS
}