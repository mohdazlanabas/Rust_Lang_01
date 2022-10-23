use std::io;

fn main() {
    println!("Hello, world!");
    
    // mutable
    let mut x = 5;
    print!("The value of x is: {x}\n");
    x=6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS:u32 = 60 * 60  * 3;
    
    // Constants
    println!("Three Hours In Secods: {THREE_HOURS_IN_SECONDS}");
    
    // Shadow
    let y = 10;
    let y = y * 2;
    {
        let y = y*2;
        println!("The value of y is: {y}");
    }
    println!("The value of y is: {y}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The spaces are {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess is: {guess}");

    // floating
    let a = 2.0;
    let b:f32 = 3.0;
    println!("The a is: {a}");
    println!("The b is: {b}");
    
    // booleam
    let t = true;
    let f: bool = false;
    println!("The t is: {t}");
    println!("The f is: {f}");

    let c ='z';
    let d: char = 'Z';
    let cat = '😻';
    println!("The c is: {c}");
    println!("The d is: {d}");
    println!("The cat is: {cat}");

    // tuple

    let tup:(i32, f64, u8, char) =(500, 6.4,1, 'e');
    let(g,h,i,e) = tup;
    println!("The value of the tupple is {g}, {h}, {i}, {e}. ", );

    // array

    let j = [1,2,3,4,5,6];
    println!("This is an array: {:?}", j);

    let k: [i32; 3] = [1,2,3];
    println!("This is an array k: {:?}", k);

    let m = [4,5,6,7];
    let first = m[0];
    let second = m[1];
    println!("This is an array m: {:?}", m);
    println!("First and Second: {first}, {second}");

    let n = [0,1,2,3,4,5];
    println!("The Array n is: {:?}", n);
    println!("Please enter an array index ?");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index was not a number");

    let element = n[index];
    println!("The value of the elemnt at index {index} is: {element}.");

    another_function();

}

fn another_function() {
    println!("Another function!");
}
